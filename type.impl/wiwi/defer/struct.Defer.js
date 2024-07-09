(function() {var type_impls = {
"wiwi":[["<details class=\"toggle implementors-toggle\" open><summary><section id=\"impl-AsMut%3CU%3E-for-Defer%3CT,+W,+F%3E\" class=\"impl\"><a class=\"src rightside\" href=\"src/wiwi/defer/mod.rs.html#448-458\">source</a><a href=\"#impl-AsMut%3CU%3E-for-Defer%3CT,+W,+F%3E\" class=\"anchor\">§</a><h3 class=\"code-header\">impl&lt;T, W, F, U&gt; <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/convert/trait.AsMut.html\" title=\"trait core::convert::AsMut\">AsMut</a>&lt;U&gt; for <a class=\"struct\" href=\"wiwi/defer/struct.Defer.html\" title=\"struct wiwi::defer::Defer\">Defer</a>&lt;T, W, F&gt;<div class=\"where\">where\n    T: <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/convert/trait.AsMut.html\" title=\"trait core::convert::AsMut\">AsMut</a>&lt;U&gt;,\n    W: When,\n    F: <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/ops/function/trait.FnOnce.html\" title=\"trait core::ops::function::FnOnce\">FnOnce</a>(T),</div></h3></section></summary><div class=\"impl-items\"><details class=\"toggle method-toggle\" open><summary><section id=\"method.as_mut\" class=\"method trait-impl\"><a class=\"src rightside\" href=\"src/wiwi/defer/mod.rs.html#455-457\">source</a><a href=\"#method.as_mut\" class=\"anchor\">§</a><h4 class=\"code-header\">fn <a href=\"https://doc.rust-lang.org/nightly/core/convert/trait.AsMut.html#tymethod.as_mut\" class=\"fn\">as_mut</a>(&amp;mut self) -&gt; <a class=\"primitive\" href=\"https://doc.rust-lang.org/nightly/std/primitive.reference.html\">&amp;mut U</a></h4></section></summary><div class='docblock'>Converts this type into a mutable reference of the (usually inferred) input type.</div></details></div></details>","AsMut<U>","wiwi::defer::DeferAlways","wiwi::defer::DeferSuccess","wiwi::defer::DeferUnwind","wiwi::defer::DeferRuntime","wiwi::defer::DeferRuntimeFn"],["<details class=\"toggle implementors-toggle\" open><summary><section id=\"impl-AsRef%3CU%3E-for-Defer%3CT,+W,+F%3E\" class=\"impl\"><a class=\"src rightside\" href=\"src/wiwi/defer/mod.rs.html#436-446\">source</a><a href=\"#impl-AsRef%3CU%3E-for-Defer%3CT,+W,+F%3E\" class=\"anchor\">§</a><h3 class=\"code-header\">impl&lt;T, W, F, U&gt; <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/convert/trait.AsRef.html\" title=\"trait core::convert::AsRef\">AsRef</a>&lt;U&gt; for <a class=\"struct\" href=\"wiwi/defer/struct.Defer.html\" title=\"struct wiwi::defer::Defer\">Defer</a>&lt;T, W, F&gt;<div class=\"where\">where\n    T: <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/convert/trait.AsRef.html\" title=\"trait core::convert::AsRef\">AsRef</a>&lt;U&gt;,\n    W: When,\n    F: <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/ops/function/trait.FnOnce.html\" title=\"trait core::ops::function::FnOnce\">FnOnce</a>(T),</div></h3></section></summary><div class=\"impl-items\"><details class=\"toggle method-toggle\" open><summary><section id=\"method.as_ref\" class=\"method trait-impl\"><a class=\"src rightside\" href=\"src/wiwi/defer/mod.rs.html#443-445\">source</a><a href=\"#method.as_ref\" class=\"anchor\">§</a><h4 class=\"code-header\">fn <a href=\"https://doc.rust-lang.org/nightly/core/convert/trait.AsRef.html#tymethod.as_ref\" class=\"fn\">as_ref</a>(&amp;self) -&gt; <a class=\"primitive\" href=\"https://doc.rust-lang.org/nightly/std/primitive.reference.html\">&amp;U</a></h4></section></summary><div class='docblock'>Converts this type into a shared reference of the (usually inferred) input type.</div></details></div></details>","AsRef<U>","wiwi::defer::DeferAlways","wiwi::defer::DeferSuccess","wiwi::defer::DeferUnwind","wiwi::defer::DeferRuntime","wiwi::defer::DeferRuntimeFn"],["<details class=\"toggle implementors-toggle\" open><summary><section id=\"impl-Debug-for-Defer%3CT,+W,+F%3E\" class=\"impl\"><a class=\"src rightside\" href=\"src/wiwi/defer/mod.rs.html#410-422\">source</a><a href=\"#impl-Debug-for-Defer%3CT,+W,+F%3E\" class=\"anchor\">§</a><h3 class=\"code-header\">impl&lt;T, W, F&gt; <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html\" title=\"trait core::fmt::Debug\">Debug</a> for <a class=\"struct\" href=\"wiwi/defer/struct.Defer.html\" title=\"struct wiwi::defer::Defer\">Defer</a>&lt;T, W, F&gt;<div class=\"where\">where\n    T: <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html\" title=\"trait core::fmt::Debug\">Debug</a>,\n    W: When + <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html\" title=\"trait core::fmt::Debug\">Debug</a>,\n    F: <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/ops/function/trait.FnOnce.html\" title=\"trait core::ops::function::FnOnce\">FnOnce</a>(T),</div></h3></section></summary><div class=\"impl-items\"><details class=\"toggle method-toggle\" open><summary><section id=\"method.fmt\" class=\"method trait-impl\"><a class=\"src rightside\" href=\"src/wiwi/defer/mod.rs.html#416-421\">source</a><a href=\"#method.fmt\" class=\"anchor\">§</a><h4 class=\"code-header\">fn <a href=\"https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html#tymethod.fmt\" class=\"fn\">fmt</a>(&amp;self, f: &amp;mut <a class=\"struct\" href=\"https://doc.rust-lang.org/nightly/core/fmt/struct.Formatter.html\" title=\"struct core::fmt::Formatter\">Formatter</a>&lt;'_&gt;) -&gt; <a class=\"type\" href=\"https://doc.rust-lang.org/nightly/core/fmt/type.Result.html\" title=\"type core::fmt::Result\">Result</a></h4></section></summary><div class='docblock'>Formats the value using the given formatter. <a href=\"https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html#tymethod.fmt\">Read more</a></div></details></div></details>","Debug","wiwi::defer::DeferAlways","wiwi::defer::DeferSuccess","wiwi::defer::DeferUnwind","wiwi::defer::DeferRuntime","wiwi::defer::DeferRuntimeFn"],["<details class=\"toggle implementors-toggle\" open><summary><section id=\"impl-Defer%3CT,+W,+F%3E\" class=\"impl\"><a class=\"src rightside\" href=\"src/wiwi/defer/mod.rs.html#185-260\">source</a><a href=\"#impl-Defer%3CT,+W,+F%3E\" class=\"anchor\">§</a><h3 class=\"code-header\">impl&lt;T, W, F&gt; <a class=\"struct\" href=\"wiwi/defer/struct.Defer.html\" title=\"struct wiwi::defer::Defer\">Defer</a>&lt;T, W, F&gt;<div class=\"where\">where\n    W: When,\n    F: <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/ops/function/trait.FnOnce.html\" title=\"trait core::ops::function::FnOnce\">FnOnce</a>(T),</div></h3></section></summary><div class=\"impl-items\"><details class=\"toggle method-toggle\" open><summary><section id=\"method.into_always\" class=\"method\"><a class=\"src rightside\" href=\"src/wiwi/defer/mod.rs.html#220-222\">source</a><h4 class=\"code-header\">pub fn <a href=\"wiwi/defer/struct.Defer.html#tymethod.into_always\" class=\"fn\">into_always</a>(self) -&gt; <a class=\"type\" href=\"wiwi/defer/type.DeferAlways.html\" title=\"type wiwi::defer::DeferAlways\">DeferAlways</a>&lt;T, F&gt;</h4></section></summary><div class=\"docblock\"><p>Consumes and returns an instance of <a href=\"wiwi/defer/type.DeferAlways.html\" title=\"type wiwi::defer::DeferAlways\"><code>DeferAlways</code></a> with the same\nclosure and value</p>\n</div></details><details class=\"toggle method-toggle\" open><summary><section id=\"method.into_on_success\" class=\"method\"><a class=\"src rightside\" href=\"src/wiwi/defer/mod.rs.html#227-229\">source</a><h4 class=\"code-header\">pub fn <a href=\"wiwi/defer/struct.Defer.html#tymethod.into_on_success\" class=\"fn\">into_on_success</a>(self) -&gt; <a class=\"type\" href=\"wiwi/defer/type.DeferSuccess.html\" title=\"type wiwi::defer::DeferSuccess\">DeferSuccess</a>&lt;T, F&gt;</h4></section></summary><div class=\"docblock\"><p>Consumes and returns an instance of <a href=\"wiwi/defer/type.DeferSuccess.html\" title=\"type wiwi::defer::DeferSuccess\"><code>DeferSuccess</code></a> with the same\nclosure and value</p>\n</div></details><details class=\"toggle method-toggle\" open><summary><section id=\"method.into_on_unwind\" class=\"method\"><a class=\"src rightside\" href=\"src/wiwi/defer/mod.rs.html#234-236\">source</a><h4 class=\"code-header\">pub fn <a href=\"wiwi/defer/struct.Defer.html#tymethod.into_on_unwind\" class=\"fn\">into_on_unwind</a>(self) -&gt; <a class=\"type\" href=\"wiwi/defer/type.DeferUnwind.html\" title=\"type wiwi::defer::DeferUnwind\">DeferUnwind</a>&lt;T, F&gt;</h4></section></summary><div class=\"docblock\"><p>Consumes and returns an instance of <a href=\"wiwi/defer/type.DeferUnwind.html\" title=\"type wiwi::defer::DeferUnwind\"><code>DeferUnwind</code></a> with the same\nclosure and value</p>\n</div></details><details class=\"toggle method-toggle\" open><summary><section id=\"method.into_runtime\" class=\"method\"><a class=\"src rightside\" href=\"src/wiwi/defer/mod.rs.html#241-243\">source</a><h4 class=\"code-header\">pub fn <a href=\"wiwi/defer/struct.Defer.html#tymethod.into_runtime\" class=\"fn\">into_runtime</a>(self, should_run: <a class=\"primitive\" href=\"https://doc.rust-lang.org/nightly/std/primitive.bool.html\">bool</a>) -&gt; <a class=\"type\" href=\"wiwi/defer/type.DeferRuntime.html\" title=\"type wiwi::defer::DeferRuntime\">DeferRuntime</a>&lt;T, F&gt;</h4></section></summary><div class=\"docblock\"><p>Consumes and returns an instance of <a href=\"wiwi/defer/type.DeferRuntime.html\" title=\"type wiwi::defer::DeferRuntime\"><code>DeferRuntime</code></a> with the same\nclosure and value</p>\n</div></details><details class=\"toggle method-toggle\" open><summary><section id=\"method.into_runtime_fn\" class=\"method\"><a class=\"src rightside\" href=\"src/wiwi/defer/mod.rs.html#248-259\">source</a><h4 class=\"code-header\">pub fn <a href=\"wiwi/defer/struct.Defer.html#tymethod.into_runtime_fn\" class=\"fn\">into_runtime_fn</a>&lt;Twhen, Fwhen&gt;(\n    self,\n    should_run_value: Twhen,\n    should_run: Fwhen,\n) -&gt; <a class=\"type\" href=\"wiwi/defer/type.DeferRuntimeFn.html\" title=\"type wiwi::defer::DeferRuntimeFn\">DeferRuntimeFn</a>&lt;T, Twhen, F, Fwhen&gt;<div class=\"where\">where\n    Fwhen: <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/ops/function/trait.FnOnce.html\" title=\"trait core::ops::function::FnOnce\">FnOnce</a>(Twhen) -&gt; <a class=\"primitive\" href=\"https://doc.rust-lang.org/nightly/std/primitive.bool.html\">bool</a>,</div></h4></section></summary><div class=\"docblock\"><p>Consumes and returns an instance of <a href=\"wiwi/defer/type.DeferRuntimeFn.html\" title=\"type wiwi::defer::DeferRuntimeFn\"><code>DeferRuntimeFn</code></a> with the same\nclosure and value</p>\n</div></details></div></details>",0,"wiwi::defer::DeferAlways","wiwi::defer::DeferSuccess","wiwi::defer::DeferUnwind","wiwi::defer::DeferRuntime","wiwi::defer::DeferRuntimeFn"],["<details class=\"toggle implementors-toggle\" open><summary><section id=\"impl-Deref-for-Defer%3CT,+W,+F%3E\" class=\"impl\"><a class=\"src rightside\" href=\"src/wiwi/defer/mod.rs.html#368-379\">source</a><a href=\"#impl-Deref-for-Defer%3CT,+W,+F%3E\" class=\"anchor\">§</a><h3 class=\"code-header\">impl&lt;T, W, F&gt; <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/ops/deref/trait.Deref.html\" title=\"trait core::ops::deref::Deref\">Deref</a> for <a class=\"struct\" href=\"wiwi/defer/struct.Defer.html\" title=\"struct wiwi::defer::Defer\">Defer</a>&lt;T, W, F&gt;<div class=\"where\">where\n    W: When,\n    F: <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/ops/function/trait.FnOnce.html\" title=\"trait core::ops::function::FnOnce\">FnOnce</a>(T),</div></h3></section></summary><div class=\"impl-items\"><details class=\"toggle\" open><summary><section id=\"associatedtype.Target\" class=\"associatedtype trait-impl\"><a href=\"#associatedtype.Target\" class=\"anchor\">§</a><h4 class=\"code-header\">type <a href=\"https://doc.rust-lang.org/nightly/core/ops/deref/trait.Deref.html#associatedtype.Target\" class=\"associatedtype\">Target</a> = T</h4></section></summary><div class='docblock'>The resulting type after dereferencing.</div></details><details class=\"toggle method-toggle\" open><summary><section id=\"method.deref\" class=\"method trait-impl\"><a class=\"src rightside\" href=\"src/wiwi/defer/mod.rs.html#376-378\">source</a><a href=\"#method.deref\" class=\"anchor\">§</a><h4 class=\"code-header\">fn <a href=\"https://doc.rust-lang.org/nightly/core/ops/deref/trait.Deref.html#tymethod.deref\" class=\"fn\">deref</a>(&amp;self) -&gt; <a class=\"primitive\" href=\"https://doc.rust-lang.org/nightly/std/primitive.reference.html\">&amp;T</a></h4></section></summary><div class='docblock'>Dereferences the value.</div></details></div></details>","Deref","wiwi::defer::DeferAlways","wiwi::defer::DeferSuccess","wiwi::defer::DeferUnwind","wiwi::defer::DeferRuntime","wiwi::defer::DeferRuntimeFn"],["<details class=\"toggle implementors-toggle\" open><summary><section id=\"impl-DerefMut-for-Defer%3CT,+W,+F%3E\" class=\"impl\"><a class=\"src rightside\" href=\"src/wiwi/defer/mod.rs.html#381-390\">source</a><a href=\"#impl-DerefMut-for-Defer%3CT,+W,+F%3E\" class=\"anchor\">§</a><h3 class=\"code-header\">impl&lt;T, W, F&gt; <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/ops/deref/trait.DerefMut.html\" title=\"trait core::ops::deref::DerefMut\">DerefMut</a> for <a class=\"struct\" href=\"wiwi/defer/struct.Defer.html\" title=\"struct wiwi::defer::Defer\">Defer</a>&lt;T, W, F&gt;<div class=\"where\">where\n    W: When,\n    F: <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/ops/function/trait.FnOnce.html\" title=\"trait core::ops::function::FnOnce\">FnOnce</a>(T),</div></h3></section></summary><div class=\"impl-items\"><details class=\"toggle method-toggle\" open><summary><section id=\"method.deref_mut\" class=\"method trait-impl\"><a class=\"src rightside\" href=\"src/wiwi/defer/mod.rs.html#387-389\">source</a><a href=\"#method.deref_mut\" class=\"anchor\">§</a><h4 class=\"code-header\">fn <a href=\"https://doc.rust-lang.org/nightly/core/ops/deref/trait.DerefMut.html#tymethod.deref_mut\" class=\"fn\">deref_mut</a>(&amp;mut self) -&gt; <a class=\"primitive\" href=\"https://doc.rust-lang.org/nightly/std/primitive.reference.html\">&amp;mut T</a></h4></section></summary><div class='docblock'>Mutably dereferences the value.</div></details></div></details>","DerefMut","wiwi::defer::DeferAlways","wiwi::defer::DeferSuccess","wiwi::defer::DeferUnwind","wiwi::defer::DeferRuntime","wiwi::defer::DeferRuntimeFn"],["<details class=\"toggle implementors-toggle\" open><summary><section id=\"impl-Display-for-Defer%3CT,+W,+F%3E\" class=\"impl\"><a class=\"src rightside\" href=\"src/wiwi/defer/mod.rs.html#424-434\">source</a><a href=\"#impl-Display-for-Defer%3CT,+W,+F%3E\" class=\"anchor\">§</a><h3 class=\"code-header\">impl&lt;T, W, F&gt; <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/fmt/trait.Display.html\" title=\"trait core::fmt::Display\">Display</a> for <a class=\"struct\" href=\"wiwi/defer/struct.Defer.html\" title=\"struct wiwi::defer::Defer\">Defer</a>&lt;T, W, F&gt;<div class=\"where\">where\n    T: <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/fmt/trait.Display.html\" title=\"trait core::fmt::Display\">Display</a>,\n    W: When,\n    F: <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/ops/function/trait.FnOnce.html\" title=\"trait core::ops::function::FnOnce\">FnOnce</a>(T),</div></h3></section></summary><div class=\"impl-items\"><details class=\"toggle method-toggle\" open><summary><section id=\"method.fmt\" class=\"method trait-impl\"><a class=\"src rightside\" href=\"src/wiwi/defer/mod.rs.html#431-433\">source</a><a href=\"#method.fmt\" class=\"anchor\">§</a><h4 class=\"code-header\">fn <a href=\"https://doc.rust-lang.org/nightly/core/fmt/trait.Display.html#tymethod.fmt\" class=\"fn\">fmt</a>(&amp;self, f: &amp;mut <a class=\"struct\" href=\"https://doc.rust-lang.org/nightly/core/fmt/struct.Formatter.html\" title=\"struct core::fmt::Formatter\">Formatter</a>&lt;'_&gt;) -&gt; <a class=\"type\" href=\"https://doc.rust-lang.org/nightly/core/fmt/type.Result.html\" title=\"type core::fmt::Result\">Result</a></h4></section></summary><div class='docblock'>Formats the value using the given formatter. <a href=\"https://doc.rust-lang.org/nightly/core/fmt/trait.Display.html#tymethod.fmt\">Read more</a></div></details></div></details>","Display","wiwi::defer::DeferAlways","wiwi::defer::DeferSuccess","wiwi::defer::DeferUnwind","wiwi::defer::DeferRuntime","wiwi::defer::DeferRuntimeFn"],["<details class=\"toggle implementors-toggle\" open><summary><section id=\"impl-Drop-for-Defer%3CT,+W,+F%3E\" class=\"impl\"><a class=\"src rightside\" href=\"src/wiwi/defer/mod.rs.html#392-408\">source</a><a href=\"#impl-Drop-for-Defer%3CT,+W,+F%3E\" class=\"anchor\">§</a><h3 class=\"code-header\">impl&lt;T, W, F&gt; <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/ops/drop/trait.Drop.html\" title=\"trait core::ops::drop::Drop\">Drop</a> for <a class=\"struct\" href=\"wiwi/defer/struct.Defer.html\" title=\"struct wiwi::defer::Defer\">Defer</a>&lt;T, W, F&gt;<div class=\"where\">where\n    W: When,\n    F: <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/ops/function/trait.FnOnce.html\" title=\"trait core::ops::function::FnOnce\">FnOnce</a>(T),</div></h3></section></summary><div class=\"impl-items\"><details class=\"toggle method-toggle\" open><summary><section id=\"method.drop\" class=\"method trait-impl\"><a class=\"src rightside\" href=\"src/wiwi/defer/mod.rs.html#398-407\">source</a><a href=\"#method.drop\" class=\"anchor\">§</a><h4 class=\"code-header\">fn <a href=\"https://doc.rust-lang.org/nightly/core/ops/drop/trait.Drop.html#tymethod.drop\" class=\"fn\">drop</a>(&amp;mut self)</h4></section></summary><div class='docblock'>Executes the destructor for this type. <a href=\"https://doc.rust-lang.org/nightly/core/ops/drop/trait.Drop.html#tymethod.drop\">Read more</a></div></details></div></details>","Drop","wiwi::defer::DeferAlways","wiwi::defer::DeferSuccess","wiwi::defer::DeferUnwind","wiwi::defer::DeferRuntime","wiwi::defer::DeferRuntimeFn"]]
};if (window.register_type_impls) {window.register_type_impls(type_impls);} else {window.pending_type_impls = type_impls;}})()