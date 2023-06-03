# (Incomplete) example of a full-stack library with WASM-frontend

This was meant to be an example of a full-stack library with WASM-frontend. However, it's incomplete because the needs of the commercial project that would benefit from the existence of this example have changed. I'm leaving this here for now, but I'm not sure when if ever I'll finish it.

## What is a full-stack library in this context?

In this context, a full-stack library is a stipulative term for a library that is meant to be included in both the frontend and the backend \[with different compile features\].

These kinds of libraries are very special because the reason why they exist in the first place is that there are situations where both the frontend and the backend take part in achieving the common goal and their code is tightly-coupled. For example, a library that encodes the data in the backend and decodes it in the frontend from a web interface, such as `ReadableStream` ( [MDN](https://developer.mozilla.org/en-US/docs/Web/API/ReadableStream) | [`web_sys`](https://rustwasm.github.io/wasm-bindgen/api/web_sys/struct.ReadableStream.html) ).

## About the relation of the targets of the frontend and the backend

This document proposes the convention that full-stack libraries should have `frontend` and `backend` features in `Cargo.toml`. This way, WASM-modules written in Rust can include the library with `frontend` feature enabled and the backend can include the library with `backend` feature enabled. Naturally, the frontend and the backend may run on different targets with different endianness, word and pointer size, etc. This is why it's important to make sure that the data sent between the frontend and the backend can be unambiguously interpreted by both sides.

## About testing of full-stack libraries

Due to sheer abundance of possible pitfalls, full-stack libraries are especially important to test. Setting up convenient testing, however, is not as easy as one could wish.

### Unit testing

#### Unit-testing for the frontend (`wasm32-unknown-unknown` target)

While testing the frontend (with `wasm32-unknown-unknown` target) using `cargo test` is possible, it's difficult to keep the necessary [WebDrivers](https://www.selenium.dev/documentation/webdriver/) up-to-date. For example, the version of [ChromeDriver](https://chromedriver.chromium.org/downloads) is required to match the version of Chrome browser.

On the other hand, [`wasm-pack`](https://rustwasm.github.io/wasm-pack/) (according to "The `wasm-bindgen` guide", "4. Testing with wasm-bindgen-test", "4.1 Usage", ["Appendix: Using wasm-bindgen-test without wasm-pack"](https://rustwasm.github.io/wasm-bindgen/wasm-bindgen-test/usage.html#appendix-using-wasm-bindgen-test-without-wasm-pack))  takes care of installing the test runner, installing a WebDriver client for your browser, and informing `cargo` how to use the custom test runner \[for `wasm32-unknown-unknown` target\]. Later, it will be discussed how to leverage `cargo make` in order to unify testing of the frontend with testing of the backend.

#### (Unit-)testing for the backend (host target)

If the backend is the host, then testing is as simple as running `cargo test` with the arguments specifying the subset of the tests.

#### (Unit-)testing for the backend (non-host target)

If the target of the backend is not the host target, then it's an open problem.

Possibly, one solution is utilization of [QEMU](https://www.qemu.org/).

#### Unified unit-testing

Check out the ["Unification of testing in general"](#unification-of-testing-in-general) section.

### Integration testing

Integration testing is arguably the most important part of testing full-stack libraries.

One important feature of integration testing is that it can happen either on some particular target (e.g. host target) or on each target from some build matrix.

#### Integration testing (host target)

If the target of the backend matches that of the host, then running the available integration tests is as simple as running `cargo test` with the arguments specifying the respective subset of the tests. However, building the integration tests themselves is a bit more complicated because the both the frontend and the backend need to be built and the integration tests need to run the server or execute a Tauri app and then somehow run the frontend (either within a WebView process of Tauri or within WebDriver).

### Unification of testing in general

The short answer is `cargo make`. Long answer might require extra time, which is currently not available.
