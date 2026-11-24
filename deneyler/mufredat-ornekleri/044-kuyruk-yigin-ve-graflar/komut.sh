#@ video: 044-kuyruk-yigin-ve-graflar
k cargo run -q
# Kaynak (nightly rust-src): VecDeque bir dairesel tampon, BinaryHeap bir ikili öbek
ks 'S=$(rustc +nightly --print sysroot)/lib/rustlib/src/rust/library; grep -n "^/// A double-ended queue implemented with a growable ring buffer" $S/alloc/src/collections/vec_deque/mod.rs; grep -n "^/// A priority queue implemented with a binary heap" $S/alloc/src/collections/binary_heap/mod.rs'
