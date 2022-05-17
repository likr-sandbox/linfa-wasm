use js_sys::Array;
use linfa::prelude::*;
use linfa_datasets::iris;
use linfa_tsne::TSneParams;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn tsne() -> JsValue {
    let ds = iris();
    let result = TSneParams::embedding_size(2)
        .perplexity(10.0)
        .approx_threshold(0.1)
        .transform(ds)
        .unwrap();
    let array = Array::new();
    for (x, y) in result.sample_iter() {
        let obj = Array::new();
        obj.push(&x[0].into());
        obj.push(&x[1].into());
        obj.push(&y[0].into());
        array.push(&obj);
    }
    array.into()
}
