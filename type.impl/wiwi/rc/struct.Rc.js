(function() {
    var type_impls = Object.fromEntries([["wiwi",[["<details class=\"toggle implementors-toggle\" open><summary><section id=\"impl-Drop-for-Rc%3CT,+U,+A%3E\" class=\"impl\"><a class=\"src rightside\" href=\"src/wiwi/rc.rs.html#183-200\">Source</a><a href=\"#impl-Drop-for-Rc%3CT,+U,+A%3E\" class=\"anchor\">§</a><h3 class=\"code-header\">impl&lt;T, U, A: CounterAccess&gt; <a class=\"trait\" href=\"wiwi/prelude_std/trait.Drop.html\" title=\"trait wiwi::prelude_std::Drop\">Drop</a> for <a class=\"struct\" href=\"wiwi/rc/struct.Rc.html\" title=\"struct wiwi::rc::Rc\">Rc</a>&lt;T, U, A&gt;</h3></section></summary><div class=\"impl-items\"><details class=\"toggle method-toggle\" open><summary><section id=\"method.drop\" class=\"method trait-impl\"><a class=\"src rightside\" href=\"src/wiwi/rc.rs.html#185-199\">Source</a><a href=\"#method.drop\" class=\"anchor\">§</a><h4 class=\"code-header\">fn <a href=\"wiwi/prelude_std/trait.Drop.html#tymethod.drop\" class=\"fn\">drop</a>(&amp;mut self)</h4></section></summary><div class='docblock'>Executes the destructor for this type. <a href=\"wiwi/prelude_std/trait.Drop.html#tymethod.drop\">Read more</a></div></details></div></details>","Drop","wiwi::rc::Arc"],["<details class=\"toggle implementors-toggle\" open><summary><section id=\"impl-Rc%3C(),+U,+A%3E\" class=\"impl\"><a class=\"src rightside\" href=\"src/wiwi/rc.rs.html#47-71\">Source</a><a href=\"#impl-Rc%3C(),+U,+A%3E\" class=\"anchor\">§</a><h3 class=\"code-header\">impl&lt;U, A: CounterAccess&gt; <a class=\"struct\" href=\"wiwi/rc/struct.Rc.html\" title=\"struct wiwi::rc::Rc\">Rc</a>&lt;<a class=\"primitive\" href=\"https://doc.rust-lang.org/nightly/std/primitive.unit.html\">()</a>, U, A&gt;</h3></section></summary><div class=\"impl-items\"><section id=\"method.from_slice_copy\" class=\"method\"><a class=\"src rightside\" href=\"src/wiwi/rc.rs.html#49-54\">Source</a><h4 class=\"code-header\">pub fn <a href=\"wiwi/rc/struct.Rc.html#tymethod.from_slice_copy\" class=\"fn\">from_slice_copy</a>(slice: &amp;<a class=\"primitive\" href=\"https://doc.rust-lang.org/nightly/std/primitive.slice.html\">[U]</a>) -&gt; Self<div class=\"where\">where\n    U: <a class=\"trait\" href=\"wiwi/prelude_std/trait.Copy.html\" title=\"trait wiwi::prelude_std::Copy\">Copy</a>,</div></h4></section><section id=\"method.from_slice_clone\" class=\"method\"><a class=\"src rightside\" href=\"src/wiwi/rc.rs.html#57-62\">Source</a><h4 class=\"code-header\">pub fn <a href=\"wiwi/rc/struct.Rc.html#tymethod.from_slice_clone\" class=\"fn\">from_slice_clone</a>(slice: &amp;<a class=\"primitive\" href=\"https://doc.rust-lang.org/nightly/std/primitive.slice.html\">[U]</a>) -&gt; Self<div class=\"where\">where\n    U: <a class=\"trait\" href=\"wiwi/prelude_std/trait.Clone.html\" title=\"trait wiwi::prelude_std::Clone\">Clone</a>,</div></h4></section><section id=\"method.from_array_into_slice\" class=\"method\"><a class=\"src rightside\" href=\"src/wiwi/rc.rs.html#65-70\">Source</a><h4 class=\"code-header\">pub fn <a href=\"wiwi/rc/struct.Rc.html#tymethod.from_array_into_slice\" class=\"fn\">from_array_into_slice</a>&lt;const N: <a class=\"primitive\" href=\"https://doc.rust-lang.org/nightly/std/primitive.usize.html\">usize</a>&gt;(array: <a class=\"primitive\" href=\"https://doc.rust-lang.org/nightly/std/primitive.array.html\">[U; N]</a>) -&gt; Self</h4></section></div></details>",0,"wiwi::rc::Arc"],["<details class=\"toggle implementors-toggle\" open><summary><section id=\"impl-Rc%3CT,+(),+A%3E\" class=\"impl\"><a class=\"src rightside\" href=\"src/wiwi/rc.rs.html#30-45\">Source</a><a href=\"#impl-Rc%3CT,+(),+A%3E\" class=\"anchor\">§</a><h3 class=\"code-header\">impl&lt;T, A: CounterAccess&gt; <a class=\"struct\" href=\"wiwi/rc/struct.Rc.html\" title=\"struct wiwi::rc::Rc\">Rc</a>&lt;T, <a class=\"primitive\" href=\"https://doc.rust-lang.org/nightly/std/primitive.unit.html\">()</a>, A&gt;</h3></section></summary><div class=\"impl-items\"><section id=\"method.new\" class=\"method\"><a class=\"src rightside\" href=\"src/wiwi/rc.rs.html#32-39\">Source</a><h4 class=\"code-header\">pub fn <a href=\"wiwi/rc/struct.Rc.html#tymethod.new\" class=\"fn\">new</a>(value: T) -&gt; Self</h4></section><section id=\"method.from_array_into_data\" class=\"method\"><a class=\"src rightside\" href=\"src/wiwi/rc.rs.html#42-44\">Source</a><h4 class=\"code-header\">pub fn <a href=\"wiwi/rc/struct.Rc.html#tymethod.from_array_into_data\" class=\"fn\">from_array_into_data</a>&lt;const N: <a class=\"primitive\" href=\"https://doc.rust-lang.org/nightly/std/primitive.usize.html\">usize</a>&gt;(array: <a class=\"primitive\" href=\"https://doc.rust-lang.org/nightly/std/primitive.array.html\">[T; N]</a>) -&gt; <a class=\"struct\" href=\"wiwi/rc/struct.Rc.html\" title=\"struct wiwi::rc::Rc\">Rc</a>&lt;<a class=\"primitive\" href=\"https://doc.rust-lang.org/nightly/std/primitive.array.html\">[T; N]</a>, <a class=\"primitive\" href=\"https://doc.rust-lang.org/nightly/std/primitive.unit.html\">()</a>, A&gt;</h4></section></div></details>",0,"wiwi::rc::Arc"],["<details class=\"toggle implementors-toggle\" open><summary><section id=\"impl-Rc%3CT,+U,+A%3E\" class=\"impl\"><a class=\"src rightside\" href=\"src/wiwi/rc.rs.html#149-181\">Source</a><a href=\"#impl-Rc%3CT,+U,+A%3E\" class=\"anchor\">§</a><h3 class=\"code-header\">impl&lt;T, U, A: CounterAccess&gt; <a class=\"struct\" href=\"wiwi/rc/struct.Rc.html\" title=\"struct wiwi::rc::Rc\">Rc</a>&lt;T, U, A&gt;</h3></section></summary><div class=\"impl-items\"><section id=\"method.downgrade\" class=\"method\"><a class=\"src rightside\" href=\"src/wiwi/rc.rs.html#151-156\">Source</a><h4 class=\"code-header\">pub fn <a href=\"wiwi/rc/struct.Rc.html#tymethod.downgrade\" class=\"fn\">downgrade</a>(this: &amp;Self) -&gt; <a class=\"struct\" href=\"wiwi/rc/struct.RcWeak.html\" title=\"struct wiwi::rc::RcWeak\">RcWeak</a>&lt;T, U, A&gt;</h4></section><section id=\"method.value\" class=\"method\"><a class=\"src rightside\" href=\"src/wiwi/rc.rs.html#159-162\">Source</a><h4 class=\"code-header\">pub fn <a href=\"wiwi/rc/struct.Rc.html#tymethod.value\" class=\"fn\">value</a>(&amp;self) -&gt; <a class=\"primitive\" href=\"https://doc.rust-lang.org/nightly/std/primitive.reference.html\">&amp;T</a></h4></section><section id=\"method.slice\" class=\"method\"><a class=\"src rightside\" href=\"src/wiwi/rc.rs.html#165-168\">Source</a><h4 class=\"code-header\">pub fn <a href=\"wiwi/rc/struct.Rc.html#tymethod.slice\" class=\"fn\">slice</a>(&amp;self) -&gt; &amp;<a class=\"primitive\" href=\"https://doc.rust-lang.org/nightly/std/primitive.slice.html\">[U]</a></h4></section><section id=\"method.strong_count\" class=\"method\"><a class=\"src rightside\" href=\"src/wiwi/rc.rs.html#171-174\">Source</a><h4 class=\"code-header\">pub fn <a href=\"wiwi/rc/struct.Rc.html#tymethod.strong_count\" class=\"fn\">strong_count</a>(&amp;self) -&gt; <a class=\"primitive\" href=\"https://doc.rust-lang.org/nightly/std/primitive.usize.html\">usize</a></h4></section><section id=\"method.weak_count\" class=\"method\"><a class=\"src rightside\" href=\"src/wiwi/rc.rs.html#177-180\">Source</a><h4 class=\"code-header\">pub fn <a href=\"wiwi/rc/struct.Rc.html#tymethod.weak_count\" class=\"fn\">weak_count</a>(&amp;self) -&gt; <a class=\"primitive\" href=\"https://doc.rust-lang.org/nightly/std/primitive.usize.html\">usize</a></h4></section></div></details>",0,"wiwi::rc::Arc"],["<details class=\"toggle implementors-toggle\" open><summary><section id=\"impl-Rc%3CT,+U,+A%3E\" class=\"impl\"><a class=\"src rightside\" href=\"src/wiwi/rc.rs.html#73-147\">Source</a><a href=\"#impl-Rc%3CT,+U,+A%3E\" class=\"anchor\">§</a><h3 class=\"code-header\">impl&lt;T, U, A: CounterAccess&gt; <a class=\"struct\" href=\"wiwi/rc/struct.Rc.html\" title=\"struct wiwi::rc::Rc\">Rc</a>&lt;T, U, A&gt;</h3></section></summary><div class=\"impl-items\"><section id=\"method.from_value_and_slice_copy\" class=\"method\"><a class=\"src rightside\" href=\"src/wiwi/rc.rs.html#75-81\">Source</a><h4 class=\"code-header\">pub fn <a href=\"wiwi/rc/struct.Rc.html#tymethod.from_value_and_slice_copy\" class=\"fn\">from_value_and_slice_copy</a>(value: T, slice: &amp;<a class=\"primitive\" href=\"https://doc.rust-lang.org/nightly/std/primitive.slice.html\">[U]</a>) -&gt; Self<div class=\"where\">where\n    U: <a class=\"trait\" href=\"wiwi/prelude_std/trait.Copy.html\" title=\"trait wiwi::prelude_std::Copy\">Copy</a>,</div></h4></section><section id=\"method.from_value_and_slice_clone\" class=\"method\"><a class=\"src rightside\" href=\"src/wiwi/rc.rs.html#108-128\">Source</a><h4 class=\"code-header\">pub fn <a href=\"wiwi/rc/struct.Rc.html#tymethod.from_value_and_slice_clone\" class=\"fn\">from_value_and_slice_clone</a>(value: T, slice: &amp;<a class=\"primitive\" href=\"https://doc.rust-lang.org/nightly/std/primitive.slice.html\">[U]</a>) -&gt; Self<div class=\"where\">where\n    U: <a class=\"trait\" href=\"wiwi/prelude_std/trait.Clone.html\" title=\"trait wiwi::prelude_std::Clone\">Clone</a>,</div></h4></section></div></details>",0,"wiwi::rc::Arc"],["<section id=\"impl-Send-for-Rc%3CT,+U,+A%3E\" class=\"impl\"><a class=\"src rightside\" href=\"src/wiwi/rc.rs.html#228\">Source</a><a href=\"#impl-Send-for-Rc%3CT,+U,+A%3E\" class=\"anchor\">§</a><h3 class=\"code-header\">impl&lt;T: <a class=\"trait\" href=\"wiwi/prelude_std/trait.Send.html\" title=\"trait wiwi::prelude_std::Send\">Send</a>, U: <a class=\"trait\" href=\"wiwi/prelude_std/trait.Send.html\" title=\"trait wiwi::prelude_std::Send\">Send</a>, A: <a class=\"trait\" href=\"wiwi/prelude_std/trait.Send.html\" title=\"trait wiwi::prelude_std::Send\">Send</a> + CounterAccess&gt; <a class=\"trait\" href=\"wiwi/prelude_std/trait.Send.html\" title=\"trait wiwi::prelude_std::Send\">Send</a> for <a class=\"struct\" href=\"wiwi/rc/struct.Rc.html\" title=\"struct wiwi::rc::Rc\">Rc</a>&lt;T, U, A&gt;</h3></section>","Send","wiwi::rc::Arc"]]]]);
    if (window.register_type_impls) {
        window.register_type_impls(type_impls);
    } else {
        window.pending_type_impls = type_impls;
    }
})()
//{"start":55,"fragment_lengths":[9918]}