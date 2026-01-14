# Configuration

RTI Connector for Rust uses an XML configuration file to define participants,
readers, writers, topics, types, and QoS. The XML schema is the same one used
by RTI Connext DDS XML-Based Application Creation.

For background on the XML format, see the
[RTI XML-Based Application Creation guide](https://community.rti.com/static/documentation/connext-dds/current/doc/manuals/connext_dds_professional/xml_application_creation/index.htm).

## Loading a configuration

Create a connector by passing a participant name and the XML file path:

```rust
use rtiddsconnector::Connector;

fn load_config() -> rtiddsconnector::ConnectorFallible {
    let _connector = Connector::new("MyLibrary::MyParticipant", "App.xml")?;
    Ok(())
}
```

The `config_name` must match a `<domain_participant>` element in the XML.

## XML tags and Connector API mapping

The table below summarizes the most common XML tags and how they map to the
Connector API:

| XML Tag | DDS Concept | Connector API |
| --- | --- | --- |
| `<types>` | Data types | Types used by `Input` and `Output` |
| `<domain_library>`, `<domain>`, `<register_type>`, `<topic>` | Domain, Topic | Defines the domain and topics used by `Connector` |
| `<domain_participant_library>`, `<domain_participant>` | DomainParticipant | Loaded by `Connector::new` |
| `<publisher>`, `<data_writer>` | Publisher, DataWriter | Each `<data_writer>` defines an `Output` |
| `<subscriber>`, `<data_reader>` | Subscriber, DataReader | Each `<data_reader>` defines an `Input` |
| `<qos_library>`, `<qos_profile>` | QoS | QoS for `Connector`, `Output`, and `Input` |

## Types

Types are defined under `<types>` and associated with topics. Example:

```xml
<types>
  <struct name="ShapeType">
    <member name="color" type="string" stringMaxLength="128" key="true"/>
    <member name="x" type="int32"/>
    <member name="y" type="int32"/>
    <member name="shapesize" type="int32"/>
  </struct>
</types>
```

You can define types in IDL and convert them to XML with `rtiddsgen`:

```
rtiddsgen -convertToXml MyTypes.idl
```

## Domain and topics

Domains register types and define topics:

```xml
<domain_library name="MyDomainLibrary">
  <domain name="MyDomain" domain_id="0">
    <register_type name="ShapeType" type_ref="ShapeType"/>
    <topic name="Square" register_type_ref="ShapeType"/>
  </domain>
</domain_library>
```

## Participants, readers, and writers

Participants contain publishers and subscribers, which in turn manage individual
writers and readers:

```xml
<domain_participant_library name="MyParticipantLibrary">
  <domain_participant name="MyPubParticipant" domain_ref="MyDomainLibrary::MyDomain">
    <publisher name="MyPublisher">
      <data_writer name="MySquareWriter" topic_ref="Square" />
    </publisher>
  </domain_participant>

  <domain_participant name="MySubParticipant" domain_ref="MyDomainLibrary::MyDomain">
    <subscriber name="MySubscriber">
      <data_reader name="MySquareReader" topic_ref="Square" />
    </subscriber>
  </domain_participant>
</domain_participant_library>
```

## QoS profiles

QoS can be configured at the profile level or per-entity. Example profile:

```xml
<qos_library name="MyQosLibrary">
  <qos_profile name="MyQosProfile" is_default_qos="true">
    <datareader_qos>
      <reliability>
        <kind>RELIABLE_RELIABILITY_QOS</kind>
      </reliability>
    </datareader_qos>
    <datawriter_qos>
      <reliability>
        <kind>RELIABLE_RELIABILITY_QOS</kind>
      </reliability>
    </datawriter_qos>
  </qos_profile>
</qos_library>
```

## Examples in this repo

This repository includes XML examples you can adapt. For an example
configuration file, see `examples/shapes/Shapes.xml`:

* `examples/MyApplication.xml`
* `examples/shapes/Shapes.xml`
