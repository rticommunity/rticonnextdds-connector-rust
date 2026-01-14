# Data access and Serde

RTI Connector exposes data as JSON under the hood, while also providing typed
accessors for common primitive fields.

## Complex types and XML definitions

The types you read and write can include nested structs, sequences, arrays, and
unions. These types are defined in XML following RTI's XML-Based Application
Creation format. For details, see
[RTI XML-Based Application Creation](https://community.rti.com/static/documentation/connext-dds/current/doc/manuals/connext_dds_professional/xml_application_creation/xml_based_app_creation_guide/UnderstandingXMLBased/XMLTagsConfigEntities.htm).

## JSON vs member access

You can access data member-by-member or as JSON. JSON access is convenient when
working with large structures; member access is convenient when you only need a
few fields.

* Set with JSON: [`crate::Instance::set_as_json`]
* Get JSON: [`crate::Sample::get_value_json`]
* Member access: [`crate::Instance::set_number`], [`crate::Instance::set_string`],
  [`crate::Sample::get_number`], [`crate::Sample::get_string`]

## Accessing basic members

Use typed setters/getters for numbers, booleans, and strings:

```rust
use rtiddsconnector::{Instance, Sample};

fn set_basic(instance: &mut Instance) -> rtiddsconnector::ConnectorFallible {
    instance.set_number("my_long", 2.0)?;
    instance.set_boolean("my_boolean", true)?;
    instance.set_string("my_string", "Hello, World!")?;
    Ok(())
}

fn get_basic(sample: &Sample) -> rtiddsconnector::ConnectorFallible {
    let _n = sample.get_number("my_long")?;
    let _b = sample.get_boolean("my_boolean")?;
    let _s = sample.get_string("my_string")?;
    Ok(())
}
```

## Accessing complex members

Examples of field-name syntax for nested members, arrays, sequences, and unions
are available in the [Accessing the data (field-name syntax examples)](https://community.rti.com/static/documentation/connector/current/api/javascript/data.html#)
chapter of the Connector for JavaScript API documentation.

## Type-independent access with SelectedValue

For dynamic access, use [`crate::Instance::set_value`] and
[`crate::Sample::get_value`], which operate on [`crate::SelectedValue`]:

```rust
use rtiddsconnector::{Instance, Sample, SelectedValue};

fn set_dynamic(instance: &mut Instance) -> rtiddsconnector::ConnectorFallible {
    instance.set_value("my_double", SelectedValue::Number(2.14))?;
    instance.set_value("my_boolean", SelectedValue::Boolean(true))?;
    instance.set_value("my_string", SelectedValue::String("Hello".to_string()))?;
    Ok(())
}

fn get_dynamic(sample: &Sample) -> rtiddsconnector::ConnectorFallible {
    let _value = sample.get_value("my_double")?;
    Ok(())
}
```

## Performance guidance

Typed getters and setters are generally faster than dynamic access with
`SelectedValue`. If you intend to access most or all members of a sample,
using JSON (`set_as_json`/`get_value_json`) can be more convenient and efficient
than setting or getting fields one by one.

## 64-bit integer limitations

RTI Connector uses a single number representation internally. This means that
64-bit integers may lose precision when converted to the internal numeric type.
If you need exact 64-bit integer values, consider representing them as strings
in your data model and converting explicitly in your application.

## Typed serialization

If you want to work with Rust structs, use Serde:

* [`crate::Instance::serialize`]: serialize a struct and set it into the
  instance.
* [`crate::Sample::deserialize`]: deserialize a sample into a struct.

These methods allow you to keep strongly-typed models in your application while
still using the dynamic RTI Connector API.
