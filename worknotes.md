anp:tldr: intern all the keys everywhere, require Hash + Eq on slots, intern id's as Vec<(&'static Location, SlotId)> -> CallId -- then the issue becomes garbage collection
anp:interning means we can have guaranteed unique ids because the original data is still stored somewhere
Aurora:The main difference I had in mind was that I was thinking interning IDs more as (CallId, &'static Location, SlotId) -> CallId
Aurora:(The first CallId being the parent)
anp:it could be reasonable for topo to be just the memoization context
anp:and then for moxie to have state, the event loop and revisions, etc
anp:the memo store for moxie is already pretty close to having a MemoStore::new().enter(|| { ... }) api
anp:that could be entered separately from the runtime, and gc'd on scope exit
Aurora:It needs to know that if a call is skipped due to memoization, all of its children are still "there".
anp:you make your ids, you get the id for your argument, and you look up the interned value for (ids, argument id) -> value
anp:i already want to make the memostore nested
anp:so that when you call memo_with! a child memostore is created
anp:and entered
anp:and gc'd as the memo init closure exits
anp:so you can have nested memoization
anp:tiered trees of side effects managed based on separate keys, etc
anp:without gc'ing everything in the nested memo context that gets skipped on later revisions
anp:so now i'm actually seeing some ways that the impl for this could actually be cleaner this way
