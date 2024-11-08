searchState.loadedDescShard("wiwi", 1, "Executes the destructor (if any) of the pointed-to value.\nCompares raw pointers for equality.\nCompares the <em>addresses</em> of the two function pointers for …\nReturns the argument unchanged.\nConverts a <code>&amp;mut T</code> to a <code>NonNull&lt;T&gt;</code>.\nReturns the argument unchanged.\nConverts a <code>&amp;T</code> to a <code>NonNull&lt;T&gt;</code>.\nReturns the argument unchanged.\nConverts a mutable reference to a raw pointer.\nConverts a mutable reference to a <code>NonNull</code> pointer.\nForms a (possibly-wide) raw pointer from a data pointer …\nPerforms the same functionality as <code>std::ptr::from_raw_parts</code>…\nPerforms the same functionality as <code>from_raw_parts</code>, except …\nConverts a reference to a raw pointer.\nConverts a reference to a <code>NonNull</code> pointer.\nReturns a raw pointer to an element or subslice, without …\nHash a raw pointer.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nReturns whether the pointer is properly aligned for <code>T</code>.\nReturns whether the pointer is aligned to <code>align</code>.\nReturns <code>true</code> if the non-null raw slice has a length of 0.\nReturns the size and alignment together as a <code>Layout</code>\nReturns the length of a non-null raw slice.\nReturns the base-2 logarithm of the alignment.\nCreates a new pointer by mapping <code>self</code>’s address to a new …\nReturns a bit mask that can be used to match this …\nExtracts the metadata component of a pointer.\nCreates a new <code>NonNull</code> if <code>ptr</code> is non-null.\nCreates an <code>Alignment</code> from a <code>usize</code>, or returns <code>None</code> if it’…\nCreates a new <code>NonNull</code>.\nCreates an <code>Alignment</code> from a power-of-two <code>usize</code>.\nCreates a null raw pointer.\nCreates a null mutable raw pointer.\nReturns the alignment for a type.\nAdds an offset to a pointer.\nCalculates the distance between two pointers within the …\nReads the value from <code>src</code> without moving it. This leaves the\nReads the value from <code>self</code> without moving it. This leaves …\nReads the value from <code>src</code> without moving it. This leaves the\nReads the value from <code>self</code> without moving it. This leaves …\nPerforms a volatile read of the value from <code>src</code> without …\nPerforms a volatile read of the value from <code>self</code> without …\nMoves <code>src</code> into the pointed <code>dst</code>, returning the previous <code>dst</code> …\nReplaces the value at <code>self</code> with <code>src</code>, returning the old …\nReturns the size of the type associated with this vtable.\nForms a raw slice from a pointer and a length.\nCreates a non-null raw slice from a thin pointer and a …\nForms a raw mutable slice from a pointer and a length.\nSubtracts an offset from a pointer (convenience for …\nCalculates the distance between two pointers within the …\nSwaps the values at two mutable locations of the same …\nSwaps the values at two mutable locations of the same …\nSwaps <code>count * size_of::&lt;T&gt;()</code> bytes between the two regions …\nDecompose a (possibly wide) pointer into its data pointer …\nCreates a new pointer with the given address and the …\nConverts an address back to a pointer, picking up some …\nConverts an address back to a mutable pointer, picking up …\nCreates a pointer with the given address and no provenance.\nCreates a pointer with the given address and no provenance.\nOverwrites a memory location with the given value without …\nOverwrites a memory location with the given value without …\nSets <code>count * size_of::&lt;T&gt;()</code> bytes of memory starting at <code>dst</code>…\nInvokes memset on the specified pointer, setting …\nOverwrites a memory location with the given value without …\nOverwrites a memory location with the given value without …\nPerforms a volatile write of a memory location with the …\nPerforms a volatile write of a memory location with the …\nContains the error value\nAn iterator over the value in a <code>Ok</code> variant of a <code>Result</code>.\nAn iterator over a reference to the <code>Ok</code> variant of a <code>Result</code>.\nAn iterator over a mutable reference to the <code>Ok</code> variant of …\nContains the success value\n<code>Result</code> is a type that represents either success (<code>Ok</code>) or …\nReturns the argument unchanged.\nReturns the argument unchanged.\nReturns the argument unchanged.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nAn iterator over a slice in (non-overlapping) chunks (<code>N</code> …\nAn iterator over a slice in (non-overlapping) mutable …\nA windowed iterator over a slice in overlapping chunks (<code>N</code> …\nAn iterator over slice in (non-overlapping) chunks …\nAn iterator over slice in (non-overlapping) mutable chunks …\nAn iterator over a slice in (non-overlapping) chunks (…\nAn iterator over a slice in (non-overlapping) chunks (…\nAn iterator over a slice in (non-overlapping) mutable …\nAn iterator over a slice in (non-overlapping) mutable …\nHelper trait for <code>[T]::concat</code>.\nAn iterator over the escaped version of a byte slice.\nImmutable slice iterator\nMutable slice iterator.\nHelper trait for <code>[T]::join</code>\nThe resulting type after concatenation\nThe resulting type after concatenation\nThe output type returned by methods.\nAn iterator over a slice in (non-overlapping) chunks (…\nAn iterator over a slice in (non-overlapping) chunks (…\nAn iterator over a slice in (non-overlapping) mutable …\nAn iterator over a slice in (non-overlapping) mutable …\nAn iterator over subslices separated by elements that …\nAn iterator over the subslices of the vector which are …\nAn iterator over subslices separated by elements that …\nAn iterator over subslices separated by elements that …\nA helper trait used for indexing operations.\nAn iterator over subslices separated by elements that …\nAn iterator over subslices separated by elements that …\nAn iterator over the mutable subslices of the vector which …\nAn iterator over the mutable subslices of the vector which …\nAn iterator over subslices separated by elements that …\nAn iterator over subslices separated by elements that …\nAn iterator over overlapping subslices of length <code>size</code>.\nViews the underlying data as a mutable subslice of the …\nViews the underlying data as a subslice of the original …\nViews the underlying data as a subslice of the original …\nReturns a slice which contains items not yet handled by …\nImplementation of <code>[T]::concat</code>\nCreates an empty slice iterator.\nCreates an empty slice iterator.\nReturns the argument unchanged.\nReturns the argument unchanged.\nReturns the argument unchanged.\nReturns the argument unchanged.\nReturns the argument unchanged.\nReturns the argument unchanged.\nReturns the argument unchanged.\nReturns the argument unchanged.\nReturns the argument unchanged.\nReturns the argument unchanged.\nReturns the argument unchanged.\nReturns the argument unchanged.\nReturns the argument unchanged.\nReturns the argument unchanged.\nReturns the argument unchanged.\nReturns the argument unchanged.\nReturns the argument unchanged.\nReturns the argument unchanged.\nReturns the argument unchanged.\nReturns the argument unchanged.\nReturns the argument unchanged.\nReturns the argument unchanged.\nReturns the argument unchanged.\nReturns the argument unchanged.\nReturns the argument unchanged.\nReturns the argument unchanged.\nReturns the argument unchanged.\nConverts a reference to T into a slice of length 1 …\nForms a mutable slice from a pointer range.\nForms a slice from a pointer range.\nForms a slice from a pointer and a length.\nPerforms the same functionality as <code>from_raw_parts</code>, except …\nConverts a reference to T into a slice of length 1 …\nReturns a shared reference to the output at this location, …\nReturns a mutable reference to the output at this …\nReturns a pointer to the output at this location, without …\nReturns a mutable pointer to the output at this location, …\nReturns a shared reference to the output at this location, …\nReturns a mutable reference to the output at this …\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nReturns the remainder of the original slice that is not …\nReturns the remainder of the original slice that is not …\nReturns the remainder of the original slice that is not …\nViews the underlying data as a subslice of the original …\nImplementation of <code>[T]::join</code>\nPerforms bounds checking of a range.\nReturns the remainder of the original slice that is not …\nReturns the remainder of the original slice that is not …\nReturns the remainder of the original slice that is not …\nPerforms bounds checking of a range without panicking.\nAn iterator over the bytes of a string slice.\nAn iterator over the <code>char</code>s of a string slice, and their …\nAn iterator over the <code>char</code>s of a string slice.\nAn iterator of <code>u16</code> over the string encoded as UTF-16.\nThe associated error which can be returned from parsing.\nThe return type of <code>str::escape_debug</code>.\nThe return type of <code>str::escape_default</code>.\nThe return type of <code>str::escape_unicode</code>.\nParse a value from a string\nAn iterator over the lines of a string, as string slices.\nCreated with the method <code>lines_any</code>.\nCreated with the method <code>match_indices</code>.\nCreated with the method <code>matches</code>.\nAn error returned when parsing a <code>bool</code> using <code>from_str</code> fails\nCreated with the method <code>rmatch_indices</code>.\nCreated with the method <code>rmatches</code>.\nCreated with the method <code>rsplit</code>.\nCreated with the method <code>rsplitn</code>.\nCreated with the method <code>rsplit_terminator</code>.\nCreated with the method <code>split</code>.\nAn iterator over the non-ASCII-whitespace substrings of a …\nAn iterator over the substrings of a string, terminated by …\nCreated with the method <code>splitn</code>.\nCreated with the method <code>split_terminator</code>.\nAn iterator over the non-whitespace substrings of a string,\nAn item returned by the <code>Utf8Chunks</code> iterator.\nAn iterator used to decode a slice of mostly UTF-8 bytes …\nErrors which can occur when attempting to interpret a …\nViews the underlying data as a subslice of the original …\nViews the underlying data as a subslice of the original …\nProvides more information about the failure:\nReturns the argument unchanged.\nReturns the argument unchanged.\nReturns the argument unchanged.\nReturns the argument unchanged.\nReturns the argument unchanged.\nReturns the argument unchanged.\nReturns the argument unchanged.\nReturns the argument unchanged.\nReturns the argument unchanged.\nReturns the argument unchanged.\nReturns the argument unchanged.\nReturns the argument unchanged.\nReturns the argument unchanged.\nReturns the argument unchanged.\nReturns the argument unchanged.\nReturns the argument unchanged.\nReturns the argument unchanged.\nReturns the argument unchanged.\nReturns the argument unchanged.\nReturns the argument unchanged.\nReturns the argument unchanged.\nReturns the argument unchanged.\nReturns the argument unchanged.\nReturns the argument unchanged.\nReturns the argument unchanged.\nReturns the argument unchanged.\nConverts a boxed slice of bytes to a boxed string slice …\nCreates a <code>&amp;str</code> from a pointer and a length.\nCreates a <code>&amp;mut str</code> from a pointer and a length.\nParses a string <code>s</code> to return a value of this type.\nConverts a slice of bytes to a string slice.\nConverts a mutable slice of bytes to a mutable string …\nConverts a slice of bytes to a string slice without …\nConverts a slice of bytes to a string slice without …\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nReturns the invalid sequence that caused a failure.\nReturns the byte position of the next character, or the …\nThe string Pattern API.\nReturns remainder of the split string.\nReturns remainder of the split string.\nReturns remainder of the split string.\nReturns remainder of the split string.\nReturns remainder of the split string.\nReturns remainder of the split string.\nReturns remainder of the split string.\nReturns the remaining lines of the split string.\nReturns remainder of the split string\nReturns remainder of the split string.\nReturns the next validated UTF-8 substring.\nReturns the index in the given string up to which valid …\nAssociated type for <code>&lt;&amp;[char; N] as Pattern&gt;::Searcher&lt;&#39;a&gt;</code>.\nAssociated type for <code>&lt;[char; N] as Pattern&gt;::Searcher&lt;&#39;a&gt;</code>.\nType returned by char types.\nAssociated type for <code>&lt;F as Pattern&gt;::Searcher&lt;&#39;a&gt;</code>.\nAssociated type for <code>&lt;char as Pattern&gt;::Searcher&lt;&#39;a&gt;</code>.\nAssociated type for <code>&lt;&amp;[char] as Pattern&gt;::Searcher&lt;&#39;a&gt;</code>.\nExpresses that every byte of the haystack has been …\nA marker trait to express that a <code>ReverseSearcher</code> can be …\nExpresses that a match of the pattern has been found at …\nA string pattern.\nExpresses that <code>haystack[a..b]</code> has been rejected as a …\nA reverse searcher for a string pattern.\nResult of calling <code>Searcher::next()</code> or …\nA searcher for a string pattern.\nAssociated searcher for this pattern\nAssociated type for <code>&lt;&amp;str as Pattern&gt;::Searcher&lt;&#39;a&gt;</code>.\nType returned by String and str types.\nResult of calling <code>Pattern::as_utf8_pattern()</code>. Can be used …\nReturns the pattern as utf-8 bytes if possible.\nReturns the argument unchanged.\nReturns the argument unchanged.\nReturns the argument unchanged.\nReturns the argument unchanged.\nReturns the argument unchanged.\nReturns the argument unchanged.\nReturns the argument unchanged.\nReturns the argument unchanged.\nGetter for the underlying string to be searched in\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nConstructs the associated searcher from <code>self</code> and the …\nChecks whether the pattern matches anywhere in the haystack\nChecks whether the pattern matches at the front of the …\nChecks whether the pattern matches at the back of the …\nPerforms the next search step starting from the front.\nPerforms the next search step starting from the back.\nFinds the next <code>Match</code> result. See <code>next()</code>.\nFinds the next <code>Match</code> result. See <code>next_back()</code>.\nFinds the next <code>Reject</code> result. See <code>next()</code> and <code>next_match()</code>.\nFinds the next <code>Reject</code> result. See <code>next_back()</code>.\nRemoves the pattern from the front of haystack, if it …\nRemoves the pattern from the back of haystack, if it …\nA draining iterator for <code>String</code>.\nA possible error value when converting a <code>String</code> from a …\nA possible error value when converting a <code>String</code> from a …\nA type alias for <code>Infallible</code>.\nA UTF-8–encoded, growable string.\nA trait for converting a value to a <code>String</code>.\nReturns a slice of <code>u8</code>s bytes that were attempted to …\nReturns the remaining (sub)string of this iterator as a …\nReturns the argument unchanged.\nReturns the argument unchanged.\nReturns the argument unchanged.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nReturns the bytes that were attempted to convert to a …\nConverts the bytes into a <code>String</code> lossily, substituting …\nConverts the given value to a <code>String</code>.\nFetch a <code>Utf8Error</code> to get more details about the conversion …\nA draining iterator for <code>Vec&lt;T&gt;</code>.\nAn iterator which uses a closure to determine if an …\nAn iterator that moves out of a vector.\nA splicing iterator for <code>Vec</code>.\nA contiguous growable array type, written as <code>Vec&lt;T&gt;</code>, short …\nReturns a reference to the underlying allocator.\nReturns a reference to the underlying allocator.\nReturns a reference to the underlying allocator.\nReturns the remaining items of this iterator as a mutable …\nReturns the remaining items of this iterator as a slice.\nReturns the remaining items of this iterator as a slice.\nCreates an empty <code>vec::IntoIter</code>.\nReturns the argument unchanged.\nReturns the argument unchanged.\nReturns the argument unchanged.\nReturns the argument unchanged.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nKeep unyielded elements in the source <code>Vec</code>.\nTrait for structs that can count references\nReference counted thin pointer, that can hold one sized …\nAtomically counted reference counting thin pointer\nWeak pointer to an atomically counted reference counted …\nAtomically counted reference counting thin pointer to a […\nWeak pointer to an atomically counted reference counted …\nSingle threaded reference counting thin pointer to a [<code>str</code>],\nWeak pointer to a single threaded reference counted thin …\nSingle threaded reference counting thin pointer\nWeak pointer to a single threaded reference counted thin …\nWeak pointer to a reference counted thin pointer <code>Rc</code>\nGets an immurable reference to the slice stored in the …\nGets an immurable reference to the value stored in the …\nCreates a new strong pointer to the same allocation, …\nCreates a new weak pointer to the same allocation, …\nDecrements the strong count during a drop, returning …\nDecrements the weak count during a drop, returning whether …\n“Downgrades” this pointer, returning a weak pointer …\nReturns the argument unchanged.\nReturns the argument unchanged.\nReturns the argument unchanged.\nReturns the argument unchanged.\nReturns the argument unchanged.\nReturns the argument unchanged.\nCreates a reference counter from an array, storing it in …\nCreates a reference counter from a slice, cloning all …\nCreates a reference counter from a slice, copying all …\nCreates a reference counter from a (sized) value, storing …\nCreates a reference counter from a value and an array, …\nCreates a reference counter from a value and a slice, …\nCreates a reference counter from a value and a slice, …\nSafety\nIncrement the strong count if it is possible to upgrade a …\nSafety\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCreate a new couter with strong and weak count both set to …\nSafety\nGets the strong pointer count\nGets the strong pointer count\n“Upgrades” this pointer, returning a strong pointer <code>Rc</code> …\nSafety\nGets the weak pointer count\nGets the weak pointer count\nSafety\nReturns the argument unchanged.\nCalls <code>U::from(self)</code>.\nThe value of a modifier, including whether or not the …\nReturns the argument unchanged.\nCalls <code>U::from(self)</code>.\nSafety\nJewel storage (a bundle of jewels)\nReturns the argument unchanged.\nCalls <code>U::from(self)</code>.")