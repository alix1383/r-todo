use cli::Comamnds;

fn main() {
    database::check_data_file_exists();

    let args = cli::cli();

    match &args.command {
        Comamnds::Add { task_name } => database::add_task(&task_name),
        Comamnds::Toggle { task_name } => database::toggle_task(&task_name),
        Comamnds::Remove { task_name } => database::remove_task(&task_name),
        Comamnds::List => database::print_task_list(),
        Comamnds::Reset => database::reset_data(),
    }
}
