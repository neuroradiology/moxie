initSidebarItems({"fn":[["current_callsite_count","Returns the number of times this callsite has been seen as a child of the current Point."]],"macro":[["call","Calls the provided expression within an [`Env`] aware to the callsite, optionally passing additional environment values to the child scope."],["callsite","Returns a value unique to the point of its invocation."],["env","Declare additional environment values to expose to a child topological function's call tree."],["root","Roots a topology at a particular callsite while calling the provided expression with the same convention as [`call`]."]],"struct":[["Callsite","A value unique to the source location where it is created."],["Env","Immutable environment container for the current (sub)topology. Environment values can be provided by parent topological invocations (currently just with [`call`] and [`root`]), but child functions can only mutate their environment through interior mutability."],["Id","Identifies an activation record in the current call topology."],["Point","The root of a sub-graph within the overall topology formed at runtime by the call-graph of topological functions."]]});