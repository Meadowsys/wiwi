(function() {
    var type_impls = Object.fromEntries([["wiwi",[["<details class=\"toggle implementors-toggle\" open><summary><section id=\"impl-Clone-for-RcWeak%3CC,+V,+S%3E\" class=\"impl\"><a class=\"src rightside\" href=\"src/wiwi/rc.rs.html#262-275\">Source</a><a href=\"#impl-Clone-for-RcWeak%3CC,+V,+S%3E\" class=\"anchor\">§</a><h3 class=\"code-header\">impl&lt;C, V, S&gt; <a class=\"trait\" href=\"wiwi/prelude/trait.Clone.html\" title=\"trait wiwi::prelude::Clone\">Clone</a> for <a class=\"struct\" href=\"wiwi/rc/struct.RcWeak.html\" title=\"struct wiwi::rc::RcWeak\">RcWeak</a>&lt;C, V, S&gt;<div class=\"where\">where\n    C: <a class=\"trait\" href=\"wiwi/rc/trait.Counter.html\" title=\"trait wiwi::rc::Counter\">Counter</a>,</div></h3></section></summary><div class=\"impl-items\"><details class=\"toggle method-toggle\" open><summary><section id=\"method.clone\" class=\"method trait-impl\"><a class=\"src rightside\" href=\"src/wiwi/rc.rs.html#269-274\">Source</a><a href=\"#method.clone\" class=\"anchor\">§</a><h4 class=\"code-header\">fn <a href=\"wiwi/prelude/trait.Clone.html#tymethod.clone\" class=\"fn\">clone</a>(&amp;self) -&gt; Self</h4></section></summary><div class=\"docblock\"><p>Creates a new weak pointer to the same allocation,\nincrementing the weak count</p>\n</div></details><details class=\"toggle method-toggle\" open><summary><section id=\"method.clone_from\" class=\"method trait-impl\"><span class=\"rightside\"><span class=\"since\" title=\"Stable since Rust version 1.0.0\">1.0.0</span> · <a class=\"src\" href=\"https://doc.rust-lang.org/nightly/src/core/clone.rs.html#174\">Source</a></span><a href=\"#method.clone_from\" class=\"anchor\">§</a><h4 class=\"code-header\">fn <a href=\"wiwi/prelude/trait.Clone.html#method.clone_from\" class=\"fn\">clone_from</a>(&amp;mut self, source: &amp;Self)</h4></section></summary><div class='docblock'>Performs copy-assignment from <code>source</code>. <a href=\"wiwi/prelude/trait.Clone.html#method.clone_from\">Read more</a></div></details></div></details>","Clone","wiwi::rc::RcThreadWeak","wiwi::rc::RcAtomicWeak"],["<details class=\"toggle implementors-toggle\" open><summary><section id=\"impl-Drop-for-RcWeak%3CC,+V,+S%3E\" class=\"impl\"><a class=\"src rightside\" href=\"src/wiwi/rc.rs.html#277-291\">Source</a><a href=\"#impl-Drop-for-RcWeak%3CC,+V,+S%3E\" class=\"anchor\">§</a><h3 class=\"code-header\">impl&lt;C, V, S&gt; <a class=\"trait\" href=\"wiwi/prelude/trait.Drop.html\" title=\"trait wiwi::prelude::Drop\">Drop</a> for <a class=\"struct\" href=\"wiwi/rc/struct.RcWeak.html\" title=\"struct wiwi::rc::RcWeak\">RcWeak</a>&lt;C, V, S&gt;<div class=\"where\">where\n    C: <a class=\"trait\" href=\"wiwi/rc/trait.Counter.html\" title=\"trait wiwi::rc::Counter\">Counter</a>,</div></h3></section></summary><div class=\"impl-items\"><details class=\"toggle method-toggle\" open><summary><section id=\"method.drop\" class=\"method trait-impl\"><a class=\"src rightside\" href=\"src/wiwi/rc.rs.html#282-290\">Source</a><a href=\"#method.drop\" class=\"anchor\">§</a><h4 class=\"code-header\">fn <a href=\"wiwi/prelude/trait.Drop.html#tymethod.drop\" class=\"fn\">drop</a>(&amp;mut self)</h4></section></summary><div class='docblock'>Executes the destructor for this type. <a href=\"wiwi/prelude/trait.Drop.html#tymethod.drop\">Read more</a></div></details></div></details>","Drop","wiwi::rc::RcThreadWeak","wiwi::rc::RcAtomicWeak"],["<details class=\"toggle implementors-toggle\" open><summary><section id=\"impl-RcWeak%3CC,+V,+S%3E\" class=\"impl\"><a class=\"src rightside\" href=\"src/wiwi/rc.rs.html#228-260\">Source</a><a href=\"#impl-RcWeak%3CC,+V,+S%3E\" class=\"anchor\">§</a><h3 class=\"code-header\">impl&lt;C, V, S&gt; <a class=\"struct\" href=\"wiwi/rc/struct.RcWeak.html\" title=\"struct wiwi::rc::RcWeak\">RcWeak</a>&lt;C, V, S&gt;<div class=\"where\">where\n    C: <a class=\"trait\" href=\"wiwi/rc/trait.Counter.html\" title=\"trait wiwi::rc::Counter\">Counter</a>,</div></h3></section></summary><div class=\"impl-items\"><details class=\"toggle method-toggle\" open><summary><section id=\"method.strong_count\" class=\"method\"><a class=\"src rightside\" href=\"src/wiwi/rc.rs.html#234-237\">Source</a><h4 class=\"code-header\">pub fn <a href=\"wiwi/rc/struct.RcWeak.html#tymethod.strong_count\" class=\"fn\">strong_count</a>(&amp;self) -&gt; <a class=\"primitive\" href=\"https://doc.rust-lang.org/nightly/std/primitive.usize.html\">usize</a></h4></section></summary><div class=\"docblock\"><p>Gets the strong pointer count</p>\n</div></details><details class=\"toggle method-toggle\" open><summary><section id=\"method.weak_count\" class=\"method\"><a class=\"src rightside\" href=\"src/wiwi/rc.rs.html#241-249\">Source</a><h4 class=\"code-header\">pub fn <a href=\"wiwi/rc/struct.RcWeak.html#tymethod.weak_count\" class=\"fn\">weak_count</a>(&amp;self) -&gt; <a class=\"primitive\" href=\"https://doc.rust-lang.org/nightly/std/primitive.usize.html\">usize</a></h4></section></summary><div class=\"docblock\"><p>Gets the weak pointer count</p>\n</div></details><details class=\"toggle method-toggle\" open><summary><section id=\"method.upgrade\" class=\"method\"><a class=\"src rightside\" href=\"src/wiwi/rc.rs.html#254-259\">Source</a><h4 class=\"code-header\">pub fn <a href=\"wiwi/rc/struct.RcWeak.html#tymethod.upgrade\" class=\"fn\">upgrade</a>(&amp;self) -&gt; <a class=\"enum\" href=\"wiwi/prelude/enum.Option.html\" title=\"enum wiwi::prelude::Option\">Option</a>&lt;<a class=\"struct\" href=\"wiwi/rc/struct.Rc.html\" title=\"struct wiwi::rc::Rc\">Rc</a>&lt;C, V, S&gt;&gt;</h4></section></summary><div class=\"docblock\"><p>“Upgrades” this pointer, returning a strong pointer <a href=\"wiwi/rc/struct.Rc.html\" title=\"struct wiwi::rc::Rc\"><code>Rc</code></a> to the data\nif there are still other strong pointers to it</p>\n</div></details></div></details>",0,"wiwi::rc::RcThreadWeak","wiwi::rc::RcAtomicWeak"],["<section id=\"impl-Send-for-RcWeak%3CC,+V,+S%3E\" class=\"impl\"><a class=\"src rightside\" href=\"src/wiwi/rc.rs.html#302-307\">Source</a><a href=\"#impl-Send-for-RcWeak%3CC,+V,+S%3E\" class=\"anchor\">§</a><h3 class=\"code-header\">impl&lt;C, V, S&gt; <a class=\"trait\" href=\"wiwi/prelude/trait.Send.html\" title=\"trait wiwi::prelude::Send\">Send</a> for <a class=\"struct\" href=\"wiwi/rc/struct.RcWeak.html\" title=\"struct wiwi::rc::RcWeak\">RcWeak</a>&lt;C, V, S&gt;<div class=\"where\">where\n    C: <a class=\"trait\" href=\"wiwi/rc/trait.Counter.html\" title=\"trait wiwi::rc::Counter\">Counter</a> + <a class=\"trait\" href=\"wiwi/prelude/trait.Send.html\" title=\"trait wiwi::prelude::Send\">Send</a>,\n    V: <a class=\"trait\" href=\"wiwi/prelude/trait.Send.html\" title=\"trait wiwi::prelude::Send\">Send</a>,\n    S: <a class=\"trait\" href=\"wiwi/prelude/trait.Send.html\" title=\"trait wiwi::prelude::Send\">Send</a>,</div></h3></section>","Send","wiwi::rc::RcThreadWeak","wiwi::rc::RcAtomicWeak"],["<section id=\"impl-Sync-for-RcWeak%3CC,+V,+S%3E\" class=\"impl\"><a class=\"src rightside\" href=\"src/wiwi/rc.rs.html#318-323\">Source</a><a href=\"#impl-Sync-for-RcWeak%3CC,+V,+S%3E\" class=\"anchor\">§</a><h3 class=\"code-header\">impl&lt;C, V, S&gt; <a class=\"trait\" href=\"wiwi/prelude/trait.Sync.html\" title=\"trait wiwi::prelude::Sync\">Sync</a> for <a class=\"struct\" href=\"wiwi/rc/struct.RcWeak.html\" title=\"struct wiwi::rc::RcWeak\">RcWeak</a>&lt;C, V, S&gt;<div class=\"where\">where\n    C: <a class=\"trait\" href=\"wiwi/rc/trait.Counter.html\" title=\"trait wiwi::rc::Counter\">Counter</a> + <a class=\"trait\" href=\"wiwi/prelude/trait.Sync.html\" title=\"trait wiwi::prelude::Sync\">Sync</a>,\n    V: <a class=\"trait\" href=\"wiwi/prelude/trait.Sync.html\" title=\"trait wiwi::prelude::Sync\">Sync</a>,\n    S: <a class=\"trait\" href=\"wiwi/prelude/trait.Sync.html\" title=\"trait wiwi::prelude::Sync\">Sync</a>,</div></h3></section>","Sync","wiwi::rc::RcThreadWeak","wiwi::rc::RcAtomicWeak"]]]]);
    if (window.register_type_impls) {
        window.register_type_impls(type_impls);
    } else {
        window.pending_type_impls = type_impls;
    }
})()
//{"start":55,"fragment_lengths":[7944]}