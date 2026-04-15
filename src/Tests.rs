#[cfg(test)]
mod tests {
    use crate::app;
    use axum_test::TestServer;

    fn server() -> TestServer {
        TestServer::new(app("generate"))
    }

    fn query_builder(pairs: &[(&str, Option<&str>)]) -> String {
        pairs
            .iter()
            .filter_map(|(k, v)| v.map(|val| format!("{}={}", k, val)))
            .collect::<Vec<_>>()
            .join("&")
    }

    // -----------------------------------------------------------------------
    // Passing tests
    // -----------------------------------------------------------------------

    #[tokio::test]
    async fn test_png_Ellers() {
        let server = server();
        let q = query_builder(&[
            ("Width", Some("10")),
            ("Height", Some("10")),
            ("Algorithm", Some("e")),
            ("Extension", Some("png")),
            ("Weighting", None),
            ("SAlgorithm", None),
        ]);
        let res = server.get(&format!("/generate?{}", q)).await;
        res.assert_status_ok();
        assert_eq!(res.headers()["content-type"], "image/png");
        assert!(!res.as_bytes().is_empty(), "PNG body must not be empty");
    }

    #[tokio::test]
    async fn test_png_GrowingTree() {
        let server = server();
        let q = query_builder(&[
            ("Width", Some("15")),
            ("Height", Some("15")),
            ("Algorithm", Some("g")),
            ("Weighting", Some("0.5")),
            ("Extension", Some("png")),
            ("SAlgorithm", None),
        ]);
        let res = server.get(&format!("/generate?{}", q)).await;
        res.assert_status_ok();
        assert_eq!(res.headers()["content-type"], "image/png");
    }

    #[tokio::test]
    async fn test_jpg_format() {
        let server = server();
        let q = query_builder(&[
            ("Width", Some("5")),
            ("Height", Some("5")),
            ("Algorithm", Some("e")),
            ("Extension", Some("jpg")),
        ]);
        let res = server.get(&format!("/generate?{}", q)).await;
        res.assert_status_ok();
        assert_eq!(res.headers()["content-type"], "image/jpg");
    }

    #[tokio::test]
    async fn test_jpeg_format() {
        let server = server();
        let q = query_builder(&[
            ("Width", Some("5")),
            ("Height", Some("5")),
            ("Algorithm", Some("e")),
            ("Extension", Some("jpeg")),
        ]);
        let res = server.get(&format!("/generate?{}", q)).await;
        res.assert_status_ok();
        assert_eq!(res.headers()["content-type"], "image/jpg");
    }

    #[tokio::test]
    async fn test_bmp_format() {
        let server = server();
        let q = query_builder(&[
            ("Width", Some("5")),
            ("Height", Some("5")),
            ("Algorithm", Some("e")),
            ("Extension", Some("bmp")),
        ]);
        let res = server.get(&format!("/generate?{}", q)).await;
        res.assert_status_ok();
        assert_eq!(res.headers()["content-type"], "image/bmp");
    }

    #[tokio::test]
    async fn test_webp_format() {
        let server = server();
        let q = query_builder(&[
            ("Width", Some("5")),
            ("Height", Some("5")),
            ("Algorithm", Some("e")),
            ("Extension", Some("webp")),
        ]);
        let res = server.get(&format!("/generate?{}", q)).await;
        res.assert_status_ok();
        assert_eq!(res.headers()["content-type"], "image/webp");
    }

    #[tokio::test]
    async fn test_tga_format() {
        let server = server();
        let q = query_builder(&[
            ("Width", Some("5")),
            ("Height", Some("5")),
            ("Algorithm", Some("e")),
            ("Extension", Some("tga")),
        ]);
        let res = server.get(&format!("/generate?{}", q)).await;
        res.assert_status_ok();
        assert_eq!(res.headers()["content-type"], "image/tga");
    }

    #[tokio::test]
    async fn test_ico_format() {
        let server = server();
        let q = query_builder(&[
            ("Width", Some("1")),
            ("Height", Some("1")),
            ("Algorithm", Some("e")),
            ("Extension", Some("ico")),
        ]);
        let res = server.get(&format!("/generate?{}", q)).await;
        res.assert_status_ok();
        assert_eq!(res.headers()["content-type"], "image/ico");
    }

    #[tokio::test]
    async fn test_avif_format() {
        let server = server();
        let q = query_builder(&[
            ("Width", Some("1")),
            ("Height", Some("1")),
            ("Algorithm", Some("e")),
            ("Extension", Some("avif")),
        ]);
        let res = server.get(&format!("/generate?{}", q)).await;
        res.assert_status_ok();
        assert_eq!(res.headers()["content-type"], "image/avif");
    }

    #[tokio::test]
    async fn test_dijkstra() {
        let server = server();
        let q = query_builder(&[
            ("Width", Some("10")),
            ("Height", Some("10")),
            ("Algorithm", Some("e")),
            ("Extension", Some("png")),
            ("SAlgorithm", Some("d")),
            ("StartX", Some("0")),
            ("StartY", Some("0")),
            ("EndX", Some("9")),
            ("EndY", Some("9")),
        ]);
        let res = server.get(&format!("/generate?{}", q)).await;
        res.assert_status_ok();
        assert_eq!(res.headers()["content-type"], "image/png");
    }

    #[tokio::test]
    async fn test_astar() {
        let server = server();
        let q = query_builder(&[
            ("Width", Some("10")),
            ("Height", Some("10")),
            ("Algorithm", Some("e")),
            ("Extension", Some("png")),
            ("SAlgorithm", Some("a")),
            ("StartX", Some("0")),
            ("StartY", Some("0")),
            ("EndX", Some("9")),
            ("EndY", Some("9")),
        ]);
        let res = server.get(&format!("/generate?{}", q)).await;
        res.assert_status_ok();
        assert_eq!(res.headers()["content-type"], "image/png");
    }

    #[tokio::test]
    async fn test_max_distance() {
        let server = server();
        let q = query_builder(&[
            ("Width", Some("20")),
            ("Height", Some("20")),
            ("Algorithm", Some("g")),
            ("Weighting", Some("0.7")),
            ("Extension", Some("png")),
            ("SAlgorithm", Some("a")),
            ("StartX", Some("0")),
            ("StartY", Some("0")),
            ("EndX", Some("19")),
            ("EndY", Some("19")),
        ]);
        let res = server.get(&format!("/generate?{}", q)).await;
        res.assert_status_ok();
    }

    #[tokio::test]
    async fn test_dimensions_min() {
        let server = server();
        let q = query_builder(&[
            ("Width", Some("1")),
            ("Height", Some("1")),
            ("Algorithm", Some("e")),
            ("Extension", Some("png")),
        ]);
        let res = server.get(&format!("/generate?{}", q)).await;
        res.assert_status_ok();
    }

    #[tokio::test]
    async fn test_dimensions_max() {
        let server = server();
        let q = query_builder(&[
            ("Width", Some("1000")),
            ("Height", Some("1000")),
            ("Algorithm", Some("e")),
            ("Extension", Some("png")),
        ]);
        let res = server.get(&format!("/generate?{}", q)).await;
        res.assert_status_ok();
    }

    #[tokio::test]
    async fn test_no_path() {
        let server = server();
        let q = query_builder(&[
            ("Width", Some("5")),
            ("Height", Some("5")),
            ("Algorithm", Some("e")),
            ("Extension", Some("png")),
            ("SAlgorithm", Some("d")),
            ("StartX", Some("2")),
            ("StartY", Some("2")),
            ("EndX", Some("2")),
            ("EndY", Some("2")),
        ]);
        let res = server.get(&format!("/generate?{}", q)).await;
        res.assert_status_ok();
    }

    #[tokio::test]
    async fn test_0_weighting() {
        let server = server();
        let q = query_builder(&[
            ("Width", Some("5")),
            ("Height", Some("5")),
            ("Algorithm", Some("g")),
            ("Weighting", Some("0.0")),
            ("Extension", Some("png")),
        ]);
        let res = server.get(&format!("/generate?{}", q)).await;
        res.assert_status_ok();
    }

    #[tokio::test]
    async fn test_1_weighting() {
        let server = server();
        let q = query_builder(&[
            ("Width", Some("5")),
            ("Height", Some("5")),
            ("Algorithm", Some("g")),
            ("Weighting", Some("1.0")),
            ("Extension", Some("png")),
        ]);
        let res = server.get(&format!("/generate?{}", q)).await;
        res.assert_status_ok();
    }

    // -----------------------------------------------------------------------
    // Exception tests
    // -----------------------------------------------------------------------

    #[tokio::test]
    async fn test_width_over_limit() {
        // width cap is 1000
        let server = server();
        let q = query_builder(&[
            ("Width", Some("1001")),
            ("Height", Some("10")),
            ("Algorithm", Some("e")),
            ("Extension", Some("png")),
        ]);
        let res = server.get(&format!("/generate?{}", q)).await;
        res.assert_status_bad_request();
    }

    #[tokio::test]
    async fn test_height_over_limit() {
        //height cap is 1000
        let server = server();
        let q = query_builder(&[
            ("Width", Some("10")),
            ("Height", Some("1001")),
            ("Algorithm", Some("e")),
            ("Extension", Some("png")),
        ]);
        let res = server.get(&format!("/generate?{}", q)).await;
        res.assert_status_bad_request();
    }

    #[tokio::test]
    async fn test_unsupported_extension() {
        let server = server();
        let q = query_builder(&[
            ("Width", Some("10")),
            ("Height", Some("10")),
            ("Algorithm", Some("e")),
            ("Extension", Some("svg")),
        ]);
        let res = server.get(&format!("/generate?{}", q)).await;
        res.assert_status_bad_request();
    }

    #[tokio::test]
    async fn test_invalid_algorithm() {
        // supported algorithms are Ellers (e) or Growing Tree (g)
        let server = server();
        let q = query_builder(&[
            ("Width", Some("10")),
            ("Height", Some("10")),
            ("Algorithm", Some("z")),
            ("Extension", Some("png")),
        ]);
        let res = server.get(&format!("/generate?{}", q)).await;
        res.assert_status_bad_request();
    }

    #[tokio::test]
    async fn test_no_weighting() {
        let server = server();
        let q = query_builder(&[
            ("Width", Some("10")),
            ("Height", Some("10")),
            ("Algorithm", Some("g")),
            ("Extension", Some("png")),
            ("Weighting", None),
        ]);
        let res = server.get(&format!("/generate?{}", q)).await;
        res.assert_status_bad_request();
    }

    #[tokio::test]
    async fn test_invalid_solver_algorithm() {
        // supported algorithms are Dijkstra's and A*
        let server = server();
        let q = query_builder(&[
            ("Width", Some("5")),
            ("Height", Some("5")),
            ("Algorithm", Some("e")),
            ("Extension", Some("png")),
            ("SAlgorithm", Some("z")),
            ("StartX", Some("0")),
            ("StartY", Some("0")),
            ("EndX", Some("4")),
            ("EndY", Some("4")),
        ]);
        let res = server.get(&format!("/generate?{}", q)).await;
        res.assert_status_bad_request();
    }

    #[tokio::test]
    async fn test_solver_out_of_bounds() {
        // max cords are (width -1, height -1)
        let server = server();
        let q = query_builder(&[
            ("Width", Some("5")),
            ("Height", Some("5")),
            ("Algorithm", Some("e")),
            ("Extension", Some("png")),
            ("SAlgorithm", Some("d")),
            ("StartX", Some("0")),
            ("StartY", Some("0")),
            ("EndX", Some("99")),
            ("EndY", Some("99")),
        ]);
        let res = server.get(&format!("/generate?{}", q)).await;
        res.assert_status_bad_request();
    }

    #[tokio::test]
    async fn test_solver_missing_start_coords() {
        let server = server();
        let q = query_builder(&[
            ("Width", Some("5")),
            ("Height", Some("5")),
            ("Algorithm", Some("e")),
            ("Extension", Some("png")),
            ("SAlgorithm", Some("d")),
            ("StartX", None),
            ("StartY", None),
            ("EndX", Some("4")),
            ("EndY", Some("4")),
        ]);
        let res = server.get(&format!("/generate?{}", q)).await;
        res.assert_status_bad_request();
    }

    #[tokio::test]
    async fn test_solver_missing_end_coords() {
        let server = server();
        let q = query_builder(&[
            ("Width", Some("5")),
            ("Height", Some("5")),
            ("Algorithm", Some("e")),
            ("Extension", Some("png")),
            ("SAlgorithm", Some("d")),
            ("StartX", Some("0")),
            ("StartY", Some("0")),
            ("EndX", None),
            ("EndY", None),
        ]);
        let res = server.get(&format!("/generate?{}", q)).await;
        res.assert_status_bad_request();
    }

    #[tokio::test]
    async fn test_ico_dimensions_over_limit() {
        // ICO max is 256x256 px. 13x13 = 262 px.
        let server = server();
        let q = query_builder(&[
            ("Width", Some("13")),
            ("Height", Some("13")),
            ("Algorithm", Some("e")),
            ("Extension", Some("ico")),
        ]);
        let res = server.get(&format!("/generate?{}", q)).await;
        res.assert_status_bad_request();
    }

    #[tokio::test]
    async fn test_webp_dimensions_over_limit() {
        // WebP max is 16383x16383. 820x820 = 16402 px.
        let server = server();
        let q = query_builder(&[
            ("Width", Some("820")),
            ("Height", Some("820")),
            ("Algorithm", Some("e")),
            ("Extension", Some("webp")),
        ]);
        let res = server.get(&format!("/generate?{}", q)).await;
        res.assert_status_bad_request();
    }

    #[tokio::test]
    async fn test_avif_dimensions_over_limit() {
        // AVIF max is 16383x16383. 820x820 = 16402 px.
        let server = server();
        let q = query_builder(&[
            ("Width", Some("820")),
            ("Height", Some("820")),
            ("Algorithm", Some("e")),
            ("Extension", Some("avif")),
        ]);
        let res = server.get(&format!("/generate?{}", q)).await;
        res.assert_status_bad_request();
    }
}
