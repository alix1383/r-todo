use cli::Comamnds;
fn main() {
    // check data dir
    database::check_data_dir();

    let args = cli::cli();
    // println!("{:#?}", args);

    match &args.command {
        Comamnds::Add { task_name } => database::add_task(&task_name),
        Comamnds::Toggle { task_name } => database::toggle_task(&task_name),
        Comamnds::Remove { task_name } => database::remove_task(&task_name),
        Comamnds::List => database::print_task_list(),
        Comamnds::Reset => database::reset_data(),
    }
}
