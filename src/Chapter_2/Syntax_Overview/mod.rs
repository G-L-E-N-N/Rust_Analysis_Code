mod control_flow_loops;
mod functions;
mod printing;
mod references;
mod structs;
mod types;
mod variables;

pub fn background_main() {
    control_flow_loops::control_flow();
    functions::functions();
    printing::printing();
    references::references();
    structs::structs();
    types::types();
    variables::variables();
}
