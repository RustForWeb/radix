// TODO: Implement
// #![cfg(target_arch = "wasm32")]
//
// use std::cell::RefCell;
// use std::rc::Rc;
//
// use leptos::*;
// use wasm_bindgen::prelude::*;
// use wasm_bindgen_test::*;
// use web_sys::Element;
//
// // Import your Avatar module:
// // (Adjust the path if your file is located elsewhere.)
// use your_crate::avatar::primitive as Avatar;
//
// wasm_bindgen_test_configure!(run_in_browser);
//
// // For test parity, define constants
// const ROOT_TEST_ID: &str = "avatar-root";
// const FALLBACK_TEXT: &str = "AB";
// const IMAGE_ALT_TEXT: &str = "Fake Avatar";
// const DELAY_MS: i32 = 300;
//
// ///////////////////////////////////////////////////////////////////////////////
// // Utility: find DOM elements
// ///////////////////////////////////////////////////////////////////////////////
//
// fn query_element_by_test_id(doc: &web_sys::Document, test_id: &str) -> Option<Element> {
//     // data-testid=...
//     doc.query_selector(&format!(r#"[data-testid="{}"]"#, test_id))
//         .ok()
//         .flatten()
// }
//
// fn query_element_by_text(doc: &web_sys::Document, text: &str) -> Option<Element> {
//     // Very naive text check
//     doc.query_selector(&format!(":contains('{}')", text)).ok().flatten()
// }
//
// fn query_img_by_alt(doc: &web_sys::Document, alt: &str) -> Option<Element> {
//     // :contains(...) won't work for alt. We do a simpler approach:
//     let imgs = doc.get_elements_by_tag_name("img");
//     for i in 0..imgs.length() {
//         let el = imgs.item(i).unwrap();
//         if el.get_attribute("alt").unwrap_or_default() == alt {
//             return Some(el);
//         }
//     }
//     None
// }
//
// ///////////////////////////////////////////////////////////////////////////////
// // Test: Avatar with fallback and no image
// ///////////////////////////////////////////////////////////////////////////////
//
// #[wasm_bindgen_test]
// fn avatar_with_fallback_no_image() {
//     // Mount the component
//     let container = leptos::document().create_element("div").unwrap();
//     leptos::document().body().unwrap().append_child(&container).unwrap();
//
//     let view = view! {
//       <Avatar::Root data-testid=ROOT_TEST_ID>
//         <Avatar::Fallback>{FALLBACK_TEXT}</Avatar::Fallback>
//       </Avatar::Root>
//     };
//
//     // Render into the container
//     leptos::mount_to(container, view);
//
//     // We can do a basic presence check
//     let doc = leptos::document();
//     let root_elem = query_element_by_test_id(&doc, ROOT_TEST_ID)
//         .expect("Should find the avatar root by test ID");
//     assert_eq!(root_elem.tag_name(), "SPAN"); // Usually <span>, because it's the Root
//
//     // Check that fallback text is present
//     let fallback_el = query_element_by_text(&doc, FALLBACK_TEXT)
//         .expect("Fallback text should be in the DOM");
//     assert_eq!(fallback_el.text_content().unwrap(), FALLBACK_TEXT);
// }
//
// ///////////////////////////////////////////////////////////////////////////////
// // Test: Avatar with fallback and a "working" image
// ///////////////////////////////////////////////////////////////////////////////
//
// // We can simulate a 'working' image by calling the onload callback ourselves.
// // Alternatively, you can define a custom <img> element in JS to mock load events.
// #[wasm_bindgen_test(async)]
// fn avatar_with_fallback_and_working_image() -> impl std::future::Future<()> {
//     use leptos::spawn_local;
//     use gloo_timers::callback::Timeout;
//
//     // We'll store references so we can check DOM after a delay
//     let container = leptos::document().create_element("div").unwrap();
//     leptos::document().body().unwrap().append_child(&container).unwrap();
//
//     let view = view! {
//       <Avatar::Root data-testid=ROOT_TEST_ID>
//         <Avatar::Fallback>{FALLBACK_TEXT}</Avatar::Fallback>
//         <Avatar::Image src="/test.jpg" alt=IMAGE_ALT_TEXT />
//       </Avatar::Root>
//     };
//     leptos::mount_to(container.clone(), view);
//
//     let doc = leptos::document();
//     let fallback_el = query_element_by_text(&doc, FALLBACK_TEXT)
//         .expect("Fallback should be in the DOM initially");
//     assert_eq!(fallback_el.text_content().unwrap(), FALLBACK_TEXT);
//
//     // The <img> should not be present yet (the load hasn't "fired")
//     {
//         let imgs = doc.get_elements_by_tag_name("img");
//         assert_eq!(imgs.length(), 0, "No image in DOM initially");
//     }
//
//     // We'll artificially "simulate" the load event after DELAY_MS
//     // Because <Avatar::Image> uses `use_event_listener` on 'load' event,
//     // we can dispatch it ourselves.
//
//     let promise = js_sys::Promise::new(&mut |resolve, _reject| {
//         let doc_cloned = doc.clone();
//         Timeout::new(DELAY_MS as u32, move || {
//             // dispatch a 'load' event to the <img>
//             let imgs = doc_cloned.get_elements_by_tag_name("img");
//             for i in 0..imgs.length() {
//                 let evt = web_sys::Event::new("load").unwrap();
//                 imgs.item(i).unwrap().dispatch_event(&evt).unwrap();
//             }
//             resolve.call0(&JsValue::NULL).unwrap();
//         })
//             .forget();
//     });
//
//     // Then we check if the fallback disappears and the image is present
//     async move {
//         wasm_bindgen_futures::JsFuture::from(promise).await.unwrap();
//
//         // Now check if an <img> is in the DOM
//         let imgs = doc.get_elements_by_tag_name("img");
//         assert_eq!(imgs.length(), 1, "Image should appear after load");
//         let image_el = imgs.item(0).unwrap();
//         assert_eq!(image_el.get_attribute("alt").unwrap_or_default(), IMAGE_ALT_TEXT);
//     }
// }
//
// ///////////////////////////////////////////////////////////////////////////////
// // Test: Avatar with fallback and delayed render
// ///////////////////////////////////////////////////////////////////////////////
//
// #[wasm_bindgen_test(async)]
// fn avatar_with_fallback_delayed_render() -> impl std::future::Future<()> {
//     use gloo_timers::callback::Timeout;
//
//     let container = leptos::document().create_element("div").unwrap();
//     leptos::document().body().unwrap().append_child(&container).unwrap();
//
//     // The fallback is delayed by DELAY_MS
//     let view = view! {
//       <Avatar::Root data-testid=ROOT_TEST_ID>
//         <Avatar::Fallback delayMs=DELAY_MS as f64>{FALLBACK_TEXT}</Avatar::Fallback>
//       </Avatar::Root>
//     };
//     leptos::mount_to(container.clone(), view);
//
//     let doc = leptos::document();
//
//     // Immediately, the fallback should NOT be present
//     assert!(
//         query_element_by_text(&doc, FALLBACK_TEXT).is_none(),
//         "Fallback should not be rendered yet"
//     );
//
//     // After ~DELAY_MS, we expect the fallback to appear
//     let promise = js_sys::Promise::new(&mut |resolve, _reject| {
//         Timeout::new(DELAY_MS as u32, move || {
//             resolve.call0(&JsValue::NULL).unwrap();
//         })
//             .forget();
//     });
//
//     async move {
//         wasm_bindgen_futures::JsFuture::from(promise).await.unwrap();
//         let fallback_el = query_element_by_text(&doc, FALLBACK_TEXT)
//             .expect("Fallback should appear after the delay");
//         assert_eq!(fallback_el.text_content().unwrap(), FALLBACK_TEXT);
//     }
// }
//
// ///////////////////////////////////////////////////////////////////////////////
// // Test: Avatar with an image that only loads when referrerPolicy="no-referrer"
// ///////////////////////////////////////////////////////////////////////////////
//
// #[wasm_bindgen_test(async)]
// fn avatar_image_referrer_policy_no_referrer() -> impl std::future::Future<()> {
//     use gloo_timers::callback::Timeout;
//
//     let container = leptos::document().create_element("div").unwrap();
//     leptos::document().body().unwrap().append_child(&container).unwrap();
//
//     let view = view! {
//       <Avatar::Root data-testid=ROOT_TEST_ID>
//         <Avatar::Fallback>{FALLBACK_TEXT}</Avatar::Fallback>
//         <Avatar::Image src="/test.jpg" alt=IMAGE_ALT_TEXT referrerPolicy="no-referrer" />
//       </Avatar::Root>
//     };
//     leptos::mount_to(container.clone(), view);
//
//     let doc = leptos::document();
//     let fallback_el = query_element_by_text(&doc, FALLBACK_TEXT)
//         .expect("Fallback should be in the DOM initially");
//     assert_eq!(fallback_el.text_content().unwrap(), FALLBACK_TEXT);
//
//     // No images initially
//     let imgs = doc.get_elements_by_tag_name("img");
//     assert_eq!(imgs.length(), 0, "Image is not loaded yet");
//
//     // We'll artificially "simulate" that only if referrerPolicy=="no-referrer" triggers 'load'
//     // Otherwise 'error'
//     let promise = js_sys::Promise::new(&mut |resolve, _reject| {
//         Timeout::new(DELAY_MS as u32, move || {
//             let imgs = doc.get_elements_by_tag_name("img");
//             for i in 0..imgs.length() {
//                 let image = imgs.item(i).unwrap();
//                 let policy = image.get_attribute("referrerPolicy").unwrap_or_default();
//                 let evt_type = if policy == "no-referrer" { "load" } else { "error" };
//                 let evt = web_sys::Event::new(evt_type).unwrap();
//                 image.dispatch_event(&evt).unwrap();
//             }
//             resolve.call0(&JsValue::NULL).unwrap();
//         })
//             .forget();
//     });
//
//     async move {
//         wasm_bindgen_futures::JsFuture::from(promise).await.unwrap();
//
//         // After the "mock" load event, we expect an <img>
//         let imgs = doc.get_elements_by_tag_name("img");
//         assert_eq!(imgs.length(), 1, "We should have 1 <img> in the DOM now");
//         let image_el = imgs.item(0).unwrap();
//         assert_eq!(image_el.get_attribute("alt").unwrap_or_default(), IMAGE_ALT_TEXT);
//     }
// }
//
// ///////////////////////////////////////////////////////////////////////////////
// // Test: Avatar with an image that breaks unless referrerPolicy="no-referrer"
// ///////////////////////////////////////////////////////////////////////////////
//
// #[wasm_bindgen_test(async)]
// fn avatar_image_referrer_policy_origin_breaks() -> impl std::future::Future<()> {
//     use gloo_timers::callback::Timeout;
//
//     let container = leptos::document().create_element("div").unwrap();
//     leptos::document().body().unwrap().append_child(&container).unwrap();
//
//     let view = view! {
//       <Avatar::Root data-testid=ROOT_TEST_ID>
//         <Avatar::Fallback>{FALLBACK_TEXT}</Avatar::Fallback>
//         <Avatar::Image src="/test.jpg" alt=IMAGE_ALT_TEXT referrerPolicy="origin" />
//       </Avatar::Root>
//     };
//     leptos::mount_to(container.clone(), view);
//
//     let doc = leptos::document();
//     let fallback_el = query_element_by_text(&doc, FALLBACK_TEXT)
//         .expect("Fallback should be visible initially");
//     assert_eq!(fallback_el.text_content().unwrap(), FALLBACK_TEXT);
//
//     let promise = js_sys::Promise::new(&mut |resolve, _reject| {
//         Timeout::new(DELAY_MS as u32, move || {
//             // We'll simulate "error" if policy != "no-referrer"
//             let imgs = doc.get_elements_by_tag_name("img");
//             for i in 0..imgs.length() {
//                 let image = imgs.item(i).unwrap();
//                 let policy = image.get_attribute("referrerPolicy").unwrap_or_default();
//                 let evt_type = if policy == "no-referrer" { "load" } else { "error" };
//                 let evt = web_sys::Event::new(evt_type).unwrap();
//                 image.dispatch_event(&evt).unwrap();
//             }
//             resolve.call0(&JsValue::NULL).unwrap();
//         })
//             .forget();
//     });
//
//     async move {
//         wasm_bindgen_futures::JsFuture::from(promise).await.unwrap();
//
//         // Because we triggered an "error" event, the <img> should never appear
//         let imgs = doc.get_elements_by_tag_name("img");
//         assert_eq!(imgs.length(), 0, "No <img> should be in the DOM after an error.");
//     }
// }
