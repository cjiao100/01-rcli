use anyhow::Result;
use axum::{
    extract::{Path, State},
    http::{Request, StatusCode},
    routing::get,
    serve, Router,
};
use std::{net::SocketAddr, path::PathBuf, sync::Arc};
use tokio::{fs, net::TcpListener};
use tower_http::services::ServeDir;
use tracing::{info, warn};

#[derive(Debug)]
pub struct HttpServeState {
    dir: PathBuf,
}

pub async fn process_http_serve(dir: PathBuf, port: u16) -> Result<()> {
    // 创建一个 SocketAddr，指定监听地址和端口
    let addr = SocketAddr::from(([127, 0, 0, 1], port));
    info!("Serving {:?} on port {}", dir, addr);

    // state 会被传递给每个请求处理函数
    let state = HttpServeState { dir: dir.clone() };

    let dir_service = ServeDir::new(dir)
        .append_index_html_on_directories(true)
        .precompressed_gzip()
        .precompressed_br()
        .precompressed_deflate()
        .precompressed_zstd();

    // 创建一个路由器 router 并添加路由
    // 路由器 router 会将请求映射到处理函数
    // 通过 with_state 方法将 state 传递给路由器
    // 这样在处理函数中就可以访问 state 了
    let router = Router::new()
        // route_service 是一个中间件，用于将请求映射到服务
        // .route_service("/tower", dir_service)
        // nest_service 是一个中间件，用于将请求映射到服务, 并添加前缀, 这里添加了前缀 /tower, 所以请求路径是 /tower/*, 会映射到 dir_service
        .nest_service("/tower", dir_service)
        // 这里使用了一个通配符 {*path}，可以匹配任意路径
        // 通过 Path 提取器可以提取请求路径中的参数
        // 通过 State 提取器可以提取 state
        .route("/{*path}", get(file_handler))
        // Arc 是一个原子引用计数类型，可以安全的在多线程中共享数据
        // 通过 Arc 包装 state，可以在多个请求中共享 state
        // state 最好可以使用 Arc 进行封装一下，不然每次请求都会克隆一份，如果 state 比较大，会影响性能
        // Arc 封装后，只会增加引用计数，不会克隆数据，所以性能会更好
        .with_state(Arc::new(state))
        .layer(tower_http::trace::TraceLayer::new_for_http().on_request(
            |req: &Request<_>, _: &_| {
                tracing::info!("Request: {} {}", req.method(), req.uri());
            },
        ));

    // 创建一个 TcpListener 并绑定到指定地址
    let listener = TcpListener::bind(addr).await?;

    // 启动服务器
    serve(listener, router).await?;

    Ok(())
}

// 处理函数
// State<Arc<HttpServeState>> 是一个提取器，用于提取 state, 并将其转换为 Arc<HttpServeState>
// Path<String> 是一个提取器，用于提取请求路径中的参数 path 并将其转换为 String 类型
//
// 处理函数返回一个实现了 IntoResponse trait 的类型
// IntoResponse trait 会将返回值转换为响应，这里返回一个元组 (StatusCode, String)
// StatusCode 是一个枚举类型，表示 HTTP 状态码
// String 表示响应内容
//
// 这里返回的是一个元组 (StatusCode, String)，表示响应的状态码和内容
// 如果状态码是 200，表示请求成功，内容是文件内容
// 如果状态码是 404，表示文件不存在，内容是错误信息
// 如果状态码是 400，表示请求错误，内容是错误信息
// 如果状态码是 500，表示服务器内部错误，内容是错误信息
//
// 这里使用了 async fn，表示这是一个异步函数
// 使用 async fn 需要 tokio::main 宏或者 tokio::runtime::Runtime 来运行
pub async fn file_handler(
    State(state): State<Arc<HttpServeState>>,
    Path(path): Path<String>,
) -> (StatusCode, String) {
    // 拼接文件路径, Path::new 是一个静态方法，用于创建一个 Path
    // join 是 Path 的一个方法，用于拼接路径
    let p = std::path::Path::new(&state.dir).join(path);
    info!("Reading file {:?}", p);

    // 判断文件是否存在
    if !p.exists() {
        return (
            StatusCode::NOT_FOUND,
            format!("File not found: {}", p.display()),
        );
    }

    // 判断是否是文件
    if !p.is_file() {
        return (
            StatusCode::BAD_REQUEST,
            format!("Not a file: {}", p.display()),
        );
    }

    // 读取文件内容
    match fs::read_to_string(p).await {
        Ok(content) => {
            info!("Read {} bytes", content.len());
            // 返回文件内容
            (StatusCode::OK, content)
        }
        Err(e) => {
            warn!("Read file error: {:?}", e);
            (StatusCode::INTERNAL_SERVER_ERROR, e.to_string())
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[tokio::test]
    async fn test_file_handler() {
        let state = Arc::new(HttpServeState {
            dir: PathBuf::from("."),
        });

        let path = Path("Cargo.toml".to_string());
        let (status, content) = file_handler(State(state), path).await;

        assert_eq!(status, StatusCode::OK);
        assert!(content.trim().starts_with("[package]"));
    }
}
