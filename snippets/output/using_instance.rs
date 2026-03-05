use rtiddsconnector::{self, Output};

fn using_instance(output: &Output) -> rtiddsconnector::ConnectorFallible {
    let mut instance = output.instance();

    instance.set_number("x", 10.0)?;
    instance.set_number("y", 20.0)?;
    instance.set_string("color", "red")?;

    println!("Modified instance: {}", instance);

    Ok(())
}
