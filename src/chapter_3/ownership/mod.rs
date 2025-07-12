mod moving;
mod copy_types;
mod drop;

pub fn ownership_main() {
    moving::ownership_move_simple();
    moving::ownership_move_conditional();
    copy_types::example_copy_temperature();
    copy_types::example_copy_simple();
    drop::drop_scope_simple();
    drop::drop_scope_block();
    drop::drop_password_prompt();
}
