use leo_ast::{LeoAst, ParserError};
use std::{env, fs, path::Path};

fn to_leo_ast(filepath: &Path) -> Result<String, ParserError> {
    // Loads the Leo code as a string from the given file path.
    let program_filepath = filepath.to_path_buf();
    let program_string = LeoAst::load_file(&program_filepath)?;

    // Parses the Leo file and constructs an abstract syntax tree.
    let ast = LeoAst::new(&program_filepath, &program_string)?;

    // Serializes the abstract syntax tree into JSON format.
    let serialized_ast = LeoAst::to_json_string(&ast)?;

    Ok(serialized_ast)
}

fn main() -> Result<(), ParserError> {
    // Parse the command-line arguments as strings.
    let cli_arguments = env::args().collect::<Vec<String>>();

    // Check that the correct number of command-line arguments were passed in.
    if cli_arguments.len() < 2 || cli_arguments.len() > 3 {
        eprintln!("Warning - an invalid number of command-line arguments were provided.");
        println!(
            "\nCommand-line usage:\n\n\tleo_ast {{PATH/TO/INPUT_FILENAME}}.leo {{PATH/TO/OUTPUT_DIRECTORY (optional)}}\n"
        );
        return Ok(()); // Exit innocently
    }

    // Construct the input filepath.
    let input_filepath = Path::new(&cli_arguments[1]);

    // Construct the serialized abstract syntax tree.
    let serialized_ast = to_leo_ast(&input_filepath)?;
    println!("{}", serialized_ast);

    // Determine the output directory.
    let output_directory = match cli_arguments.len() == 3 {
        true => format!(
            "{}/{}.json",
            cli_arguments[2],
            input_filepath.file_stem().unwrap().to_str().unwrap()
        ),
        false => format!("./{}.json", input_filepath.file_stem().unwrap().to_str().unwrap()),
    };

    // Write the serialized abstract syntax tree to the output directory.
    fs::write(Path::new(&output_directory), serialized_ast)?;

    Ok(())
}