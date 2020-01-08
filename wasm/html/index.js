const js = import("../pkg/sudachi_wasm_sample.js");
js.then(js => {
  const e = document.getElementById("btn");
  e.addEventListener('click', () => {
    const ee = document.getElementById("src");
    const s = js.analyze(ee.value);
    alert(s);
  });
});
