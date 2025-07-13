mod immutable_borrows;
mod mutable_borrows;
//mod dangling_references;
mod function_calls;

pub fn borrowing_main(){
  immutable_borrows::multiple_immutable_borrows_basic();
  immutable_borrows::multiple_immutable_borrows_config();
  mutable_borrows::mutable_borrow_and_modify();
  mutable_borrows::multiple_mutable_borrows_illegal();
  mutable_borrows::sorting_mut_slice_example();
  //dangling_references::dangling_reference_example();
  function_calls::pass_by_reference();
  function_calls::multiple_pass_by_reference();
}
