export RUSTFLAGS='--cfg getrandom_backend="wasm_js"'
cargo build --release --target wasm32-unknown-unknown
wasm-bindgen --out-name crossequa   --out-dir web  --target web target/wasm32-unknown-unknown/release/crossequa.wasm
echo '
<html lang="en">

<head>
    <meta charset="UTF-8" />
    <style>
        body {
            background: linear-gradient(135deg,
                    white 0%,
                    white 49%,
                    black 49%,
                    black 51%,
                    white 51%,
                    white 100%) repeat;
            background-size: 20px 20px;
        }

        canvas {
            background-color: white;
        }
    </style>
    <title>Wasm Example</title>
</head>
<script type="module">
    import init from "./crossequa.js"
    init()
</script>

</html>
' > web/index.html

basic-http-server web