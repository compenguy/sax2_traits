/// The SAX2 API definitions from which these interfaces were derived comes with
/// the following notice:
///
/// > This module, both source code and documentation, is in the Public Domain,
/// > and comes with NO WARRANTY. See http://www.saxproject.org for further
/// > information.
///
/// See also http://www.saxproject.org/copying.html
use std::rc::Rc;

use common::EntityResolver;
use common::Error;
use common::InputSource;
use common::Locator;

/// Modelled after `org.xml.sax.SAXParseException`
pub trait ParseError: Error + Locator {}

pub type Result<T> = std::result::Result<T, Box<Error>>;

/// Interface for a list of XML attributes.
///
/// This interface allows access to a list of attributes in three different ways:
///
/// 1. by attribute index;
/// 2. by Namespace-qualified name; or
/// 3. by qualified (prefixed) name.
///
/// The list will not contain attributes that were declared #IMPLIED but not specified in the
/// start tag. It will also not contain attributes used as Namespace declarations (xmlns*)
/// unless the http://xml.org/sax/features/namespace-prefixes feature is set to true (it is
/// false by default). Because SAX2 conforms to the original "Namespaces in XML" recommendation,
/// it normally does not give namespace declaration attributes a namespace URI.
///
/// Some SAX2 parsers may support using an optional feature flag
/// (http://xml.org/sax/features/xmlns-uris) to request that those attributes be given URIs,
/// conforming to a later backwards-incompatible revision of that recommendation. (The
/// attribute's "local name" will be the prefix, or "xmlns" when defining a default element
/// namespace.) For portability, handler code should always resolve that conflict, rather than
/// requiring parsers that can change the setting of that feature flag.
///
/// If the namespace-prefixes feature (see above) is false, access by qualified name may not be
/// available; if the http://xml.org/sax/features/namespaces feature is false, access by
/// Namespace-qualified names may not be available.
///
/// The order of attributes in the list is unspecified, and will vary from implementation to
/// implementation.
///
/// Modelled after `org.xml.sax.Attributes`
pub trait Attributes {
    /// Look up the index of an attribute by XML qualified (prefixed) name.
    fn get_q_name_index(&self, q_name: &str) -> Option<u64>;
    /// Look up the index of an attribute by Namespace name.
    fn get_ns_name_index(&self, uri: &str, local_name: &str) -> Option<u64>;
    /// Return the number of attributes in the list.
    fn get_length(&self) -> usize;
    /// Look up an attribute's local name by index.
    fn get_local_name(&self, index: u64) -> Option<String>;
    /// Look up an attribute's XML qualified (prefixed) name by index.
    fn get_q_name(&self, index: u64) -> Option<String>;
    /// Look up an attribute's type by index.
    ///
    /// The attribute type is one of the strings "CDATA", "ID", "IDREF", "IDREFS", "NMTOKEN",
    /// "NMTOKENS", "ENTITY", "ENTITIES", or "NOTATION" (always in upper case).
    ///
    /// If the parser has not read a declaration for the attribute, or if the parser does not
    /// report attribute types, then it must return the value "CDATA" as stated in the XML 1.0
    /// Recommendation (clause 3.3.3, "Attribute-Value Normalization").
    ///
    /// For an enumerated attribute that is not a notation, the parser will report the type as
    /// "NMTOKEN".
    fn get_type(&self, index: u64) -> Option<String>;
    /// Look up an attribute's type by XML qualified (prefixed) name.
    fn get_q_name_type(&self, q_name: &str) -> Option<String> {
        self.get_q_name_index(q_name).and_then(|i| self.get_type(i))
    }
    /// Look up an attribute's type by Namespace name.
    fn get_ns_name_type(&self, uri: &str, local_name: &str) -> Option<String> {
        self.get_ns_name_index(uri, local_name)
            .and_then(|i| self.get_type(i))
    }
    /// Look up an attribute's Namespace URI by index.
    fn get_uri(&self, index: u64) -> Option<String>;
    /// Look up an attribute's value by index.
    ///
    /// If the attribute value is a list of tokens (IDREFS, ENTITIES, or NMTOKENS), the tokens
    /// will be concatenated into a single string with each token separated by a single space.
    fn get_value(&self, index: u64) -> Option<String>;
    /// Look up an attribute's value by XML qualified (prefixed) name.
    fn get_q_name_value(&self, q_name: &str) -> Option<String> {
        self.get_q_name_index(q_name)
            .and_then(|i| self.get_value(i))
    }
    /// Look up an attribute's value by Namespace name.
    fn get_ns_name_value(&self, uri: &str, local_name: &str) -> Option<String> {
        self.get_ns_name_index(uri, local_name)
            .and_then(|i| self.get_value(i))
    }
}

/// Basic interface for SAX error handlers.
///
/// If a SAX application needs to implement customized error handling, it must implement this
/// interface and then register an instance with the XML reader using the `set_error_handler`
/// method. The parser will then report all errors and warnings through this interface.
///
/// WARNING: If an application does not register an `ErrorHandler`, XML parsing errors will go
/// unreported, except that `ParseErrors` will be thrown for fatal errors. In order to
/// detect validity errors, an `ErrorHandler` that does something with error() calls must be
/// registered.
///
/// For XML processing errors, a SAX driver must use this interface in preference to throwing an
/// exception: it is up to the application to decide whether to throw an exception for different
/// types of errors and warnings. Note, however, that there is no requirement that the parser
/// continue to report additional errors after a call to `fatal_error`. In other words, a SAX
/// driver class may throw an exception after reporting any `fatal_error`. Also parsers may throw
/// appropriate exceptions for non-XML errors. For example, `XmlReader.parse()` would throw an
/// IOException for errors accessing entities or the document.
///
/// Modelled after `org.xml.sax.ErrorHandler`
pub trait ErrorHandler<E: ParseError> {
    /// Receives notification of a recoverable parser error.
    ///
    /// This corresponds to the definition of "error" in section 1.2 of the W3C XML 1.0
    /// Recommendation. For example, a validating parser would use this callback to report the
    /// violation of a validity constraint. The default behaviour is to take no action.
    ///
    /// The SAX parser must continue to provide normal parsing events after invoking this
    /// method: it should still be possible for the application to process the document through
    /// to the end. If the application cannot do so, then the parser should report a fatal error
    /// even if the XML recommendation does not require it to do so.
    ///
    /// Filters may use this method to report other, non-XML errors as well.
    fn error(&self, e: &E) -> Result<()> {
        eprintln!("error: {}", e);
        Ok(())
    }
    /// Receives report of a fatal XML parsing error.
    ///
    /// This corresponds to the definition of "fatal error" in section 1.2 of the W3C XML 1.0
    /// Recommendation. For example, a parser would use this callback to report the violation of
    /// a well-formedness constraint.
    ///
    /// The application must assume that the document is unusable after the parser has invoked
    /// this method, and should continue (if at all) only for the sake of collecting additional
    /// error messages: in fact, SAX parsers are free to stop reporting any other events once
    /// this method has been invoked
    fn fatal_error(&self, e: &E) -> Result<()> {
        panic!("fatal error: {}", e);
    }

    /// Receive notification of a parser warning.
    ///
    /// SAX parsers will use this method to report conditions that are not errors or fatal
    /// errors as defined by the XML recommendation. The default behaviour is to take no action.
    ///
    /// The SAX parser must continue to provide normal parsing events after invoking this
    /// method: it should still be possible for the application to process the document through
    /// to the end.
    ///
    /// Filters may use this method to report other, non-XML warnings as well.
    fn warning(&self, e: &E) -> Result<()> {
        eprintln!("warning: {}", e);
        Ok(())
    }
}

/// Receives notification of basic DTD-related events.
///
/// If a SAX application needs information about notations and unparsed entities, then the
/// application implements this interface and registers an instance with the SAX parser using
/// the parser's `set_dtd_handler method`. The parser uses the instance to report notation and
/// unparsed entity declarations to the application.
///
/// Note that this interface includes only those DTD events that the XML recommendation requires
/// processors to report: notation and unparsed entity declarations.
///
/// The SAX parser may report these events in any order, regardless of the order in which the
/// notations and unparsed entities were declared; however, all DTD events must be reported
/// after the document handler's `start_document` event, and before the first `start_element` event.
/// (If the `LexicalHandler` is used, these events must also be reported before the `end_dtd` event.)
///
/// It is up to the application to store the information for future use (perhaps in a hash table
/// or object tree). If the application encounters attributes of type "NOTATION", "ENTITY", or
/// "ENTITIES", it can use the information that it obtained through this interface to find the
/// entity and/or notation corresponding with the attribute value.
///
/// Modelled after `org.xml.sax.DTDHandler`
pub trait DtdHandler {
    /// Receives notification of a notation declaration.
    ///
    /// It is up to the application to record the notation for later reference, if necessary;
    /// notations may appear as attribute values and in unparsed entity declarations, and are
    /// sometime used with processing instruction target names.
    ///
    /// At least one of `public_id` and `system_id` must be non-`None`. If a system identifier is
    /// present, and it is a URL, the SAX parser must resolve it fully before passing it to the
    /// application through this event.
    ///
    /// There is no guarantee that the notation declaration will be reported before any unparsed
    /// entities that use it.
    #[allow(unused_variables)]
    fn notation_decl(
        &self,
        name: &str,
        public_id: Option<&str>,
        system_id: Option<&str>,
    ) -> Result<()> {
        Ok(())
    }

    /// Receives notification of an unparsed entity declaration.
    ///
    /// Note that the notation name corresponds to a notation reported by the `notation_decl`
    /// event. It is up to the application to record the entity for later reference, if
    /// necessary; unparsed entities may appear as attribute values.
    ///
    /// If the system identifier is a URL, the parser must resolve it fully before passing it to
    /// the application.
    #[allow(unused_variables)]
    fn unparsed_entity_decl(
        &self,
        name: &str,
        public_id: Option<&str>,
        system_id: &str,
        notation_name: &str,
    ) -> Result<()> {
        Ok(())
    }
}

/// Receives notification of the logical content of a document.
///
/// This is the main interface that most SAX applications implement: if the application needs to
/// be informed of basic parsing events, it implements this interface and registers an instance
/// with the SAX parser using the `set_content_handler` method. The parser uses the instance to
/// report basic document-related events like the start and end of elements and character data.
///
/// The order of events in this interface is very important, and mirrors the order of
/// information in the document itself. For example, all of an element's content (character
/// data, processing instructions, and/or subelements) will appear, in order, between the
/// `start_element` event and the corresponding `end_element` event.
///
/// This interface is similar to the now-deprecated SAX 1.0 `DocumentHandler` interface, but it
/// adds support for Namespaces and for reporting skipped entities (in non-validating XML
/// processors).
pub trait ContentHandler<L: Locator, A: Attributes> {
    /// Receives notification of character data inside an element.
    ///
    /// The Parser will call this method to report each chunk of character data. SAX parsers may
    /// return all contiguous character data in a single chunk, or they may split it into
    /// several chunks; however, all of the characters in any single event must come from the
    /// same external entity so that the Locator provides useful information.
    ///
    /// The application must not attempt to read from the array outside of the specified range.
    ///
    /// Individual characters may consist of more than one utf-8 char value. There are two
    /// important cases where this happens, because characters can't be represented in just
    /// sixteen bits. In one case, characters are represented in a Surrogate Pair, using two
    /// special Unicode values. Such characters are in the so-called "Astral Planes", with a
    /// code point above U+FFFF. A second case involves composite characters, such as a base
    /// character combining with one or more accent characters.
    ///
    /// Your code should not assume that algorithms using char-at-a-time idioms will be working
    /// in character units; in some cases they will split characters. This is relevant wherever
    /// XML permits arbitrary characters, such as attribute values, processing instruction data,
    /// and comments as well as in data reported from this method. It's also generally relevant
    /// whenever code manipulates internationalized text; the issue isn't unique to XML.
    ///
    /// Note that some parsers will report whitespace in element content using the
    /// `ignorable_whitespace` method rather than this one (validating parsers must do so).
    #[allow(unused_variables)]
    fn characters(&self, content: &str) -> Result<()> {
        Ok(())
    }
    /// Receives notification of the end of the document.
    ///
    /// The SAX parser will invoke this method only once, and it will be the last method invoked
    /// during the parse. The parser shall not invoke this method until it has either abandoned
    /// parsing (because of an unrecoverable error) or reached the end of input.
    #[allow(unused_variables)]
    fn end_document(&self) -> Result<()> {
        Ok(())
    }
    /// Receives notification of the end of an element.
    ///
    /// The SAX parser will invoke this method at the end of every element in the XML document;
    /// there will be a corresponding `start_element` event for every `end_element` event (even when
    /// the element is empty).
    ///
    /// For information on the names, see `start_element`.
    #[allow(unused_variables)]
    fn end_element(&self, uri: &str, local_name: &str, q_name: &str) -> Result<()> {
        Ok(())
    }
    /// Receives notification of the end of a Namespace mapping.
    ///
    /// See `start_prefix_mapping` for details. These events will always occur immediately after
    /// the corresponding `end_element` event, but the order of `end_prefix_mapping` events is not
    /// otherwise guaranteed.
    #[allow(unused_variables)]
    fn end_prefix_mapping(&self, prefix: &str) -> Result<()> {
        Ok(())
    }
    /// Receives notification of ignorable whitespace in element content.
    ///
    /// Validating Parsers must use this method to report each chunk of whitespace in element
    /// content (see the W3C XML 1.0 recommendation, section 2.10): non-validating parsers may
    /// also use this method if they are capable of parsing and using content models.
    ///
    /// SAX parsers may return all contiguous whitespace in a single chunk, or they may split it
    /// into several chunks; however, all of the characters in any single event must come from
    /// the same external entity, so that the Locator provides useful information.
    ///
    /// The application must not attempt to read from the array outside of the specified range.
    #[allow(unused_variables)]
    fn ignorable_whitespace(&self, content: &str) -> Result<()> {
        Ok(())
    }
    /// Receives notification of a processing instruction.
    #[allow(unused_variables)]
    fn processing_instruction(&self, target: &str, data: &str) -> Result<()> {
        Ok(())
    }
    /// Receives a Locator object for document events.
    /// SAX parsers are strongly encouraged (though not absolutely required) to supply a locator: if
    /// it does so, it must supply the locator to the application by invoking this method before
    /// invoking any of the other methods in the `ContentHandler` interface.
    ///
    /// The locator allows the application to determine the end position of any document-related
    /// event, even if the parser is not reporting an error. Typically, the application will use
    /// this information for reporting its own errors (such as character content that does not match
    /// an application's business rules). The information returned by the locator is probably not
    /// sufficient for use with a search engine.
    ///
    /// Note that the locator will return correct information only during the invocation SAX event
    /// callbacks after `start_document` returns and before `end_document` is called. The application
    /// should not attempt to use it at any other time.
    ///
    /// Parameters:
    /// locator - an object that can return the location of any SAX document event
    #[allow(unused_variables)]
    fn set_document_locator(&self, locator: Rc<L>) {}
    /// Receives notification of a skipped entity.
    ///
    /// This is not called for entity references within markup constructs such as element start
    /// tags or markup declarations. (The XML recommendation requires reporting skipped external
    /// entities. SAX also reports internal entity expansion/non-expansion, except within markup
    /// constructs.)
    ///
    /// The Parser will invoke this method each time the entity is skipped. Non-validating
    /// processors may skip entities if they have not seen the declarations (because, for
    /// example, the entity was declared in an external DTD subset). All processors may skip
    /// external entities, depending on the values of the
    /// http://xml.org/sax/features/external-general-entities and the
    /// http://xml.org/sax/features/external-parameter-entities properties.
    #[allow(unused_variables)]
    fn skipped_entity(&self, name: &str) -> Result<()> {
        Ok(())
    }
    /// Receive notification of the beginning of the document.
    ///
    /// The SAX parser will invoke this method only once, before any other event callbacks
    /// (except for `set_document_locator`).
    #[allow(unused_variables)]
    fn start_document(&self) -> Result<()> {
        Ok(())
    }
    /// Receives notification of the start of an element.
    ///
    /// The Parser will invoke this method at the beginning of every element in the XML
    /// document; there will be a corresponding `end_element` event for every `start_element` event
    /// (even when the element is empty). All of the element's content will be reported, in
    /// order, before the corresponding `end_element` event.
    ///
    /// This event allows up to three name components for each element:
    ///
    /// 1. the Namespace URI;
    /// 2. the local name; and
    /// 3. the qualified (prefixed) name.
    ///
    /// Any or all of these may be provided, depending on the values of the
    /// http://xml.org/sax/features/namespaces and the
    /// http://xml.org/sax/features/namespace-prefixes properties:
    ///
    /// * the Namespace URI and local name are required when the namespaces property is true (the
    ///   default), and are optional when the namespaces property is false (if one is specified,
    /// both must be);
    /// * the qualified name is required when the namespace-prefixes property is true, and is
    ///   optional when the namespace-prefixes property is false (the default).
    ///
    /// Note that the attribute list provided will contain only attributes with explicit values
    /// (specified or defaulted): #IMPLIED attributes will be omitted. The attribute list will
    /// contain attributes used for Namespace declarations (xmlns* attributes) only if the
    /// http://xml.org/sax/features/namespace-prefixes property is true (it is false by default,
    /// and support for a true value is optional).
    ///
    /// Like characters(), attribute values may have characters that need more than one char
    /// value.
    #[allow(unused_variables)]
    fn start_element(
        &self,
        uri: &str,
        local_name: &str,
        q_name: &str,
        attributes: A,
    ) -> Result<()> {
        Ok(())
    }
    /// Receives notification of the start of a Namespace mapping.
    ///
    /// The information from this event is not necessary for normal Namespace processing: the
    /// SAX XML reader will automatically replace prefixes for element and attribute names when
    /// the http://xml.org/sax/features/namespaces feature is true (the default).
    ///
    /// There are cases, however, when applications need to use prefixes in character data or in
    /// attribute values, where they cannot safely be expanded automatically; the
    /// `start`/`end_prefix_mapping` event supplies the information to the application to expand
    /// prefixes in those contexts itself, if necessary.
    ///
    /// Note that `start`/`end_prefix_mapping` events are not guaranteed to be properly nested
    /// relative to each other: all `start_prefix_mapping` events will occur immediately before
    /// the corresponding `start_element` event, and all `end_prefix_mapping` events will occur
    /// immediately after the corresponding `end_element` event, but their order is not otherwise
    /// guaranteed.
    ///
    /// There should never be `start`/`end_prefix_mapping events` for the "xml" prefix, since it is
    /// predeclared and immutable.
    #[allow(unused_variables)]
    fn start_prefix_mapping(&self, prefix: &str, uri: &str) -> Result<()> {
        Ok(())
    }
}

/// Interface for reading an XML document using callbacks.
///
/// Note: despite its name, this interface does not implement the `Read` trait,
/// because reading XML is a fundamentally different activity than reading character data.
///
/// XmlReader is the interface that an XML parser's SAX2 driver must implement. This interface
/// allows an application to set and query features and properties in the parser, to register event
/// handlers for document processing, and to initiate a document parse.
///
/// All SAX interfaces are assumed to be synchronous: the parse methods must not return until
/// parsing is complete, and readers must wait for an event-handler callback to return before
/// reporting the next event.
pub trait XmlReader<
    CH: ContentHandler<L, A>,
    DH: DtdHandler,
    ER: EntityResolver,
    EH: ErrorHandler<E>,
    L: Locator,
    A: Attributes,
    E: ParseError,
    I: InputSource,
>
{
    /// Return the current content handler.
    fn get_content_handler(&self) -> Option<&CH>;
    /// Return the current content handler (mutable).
    fn get_content_handler_mut(&self) -> Option<&mut CH>;
    /// Return the current DTD handler.
    fn get_dtd_handler(&self) -> Option<&DH>;
    /// Return the current DTD handler (mutable).
    fn get_dtd_handler_mut(&self) -> Option<&mut DH>;
    /// Return the current entity resolver.
    fn get_entity_resolver(&self) -> Option<&ER>;
    /// Return the current entity resolver (mutable).
    fn get_entity_resolver_mut(&self) -> Option<&mut DH>;
    /// Return the current error handler.
    fn get_error_handler(&self) -> Option<&EH>;
    /// Return the current error handler (mutable).
    fn get_error_handler_mut(&self) -> Option<&mut EH>;
    /// Look up the value of a feature flag.
    ///
    /// The feature name is any fully-qualified URI. It is possible for an XmlReader to recognize a
    /// feature name but temporarily be unable to return its value. Some feature values may be
    /// available only in specific contexts, such as before, during, or after a parse. Also, some
    /// feature values may not be programmatically accessible. (In the case of an adapter for SAX1
    /// Parser, there is no implementation-independent way to expose whether the underlying parser
    /// is performing validation, expanding external entities, and so forth.)
    ///
    /// All XmlReaders are required to recognize the http://xml.org/sax/features/namespaces and the
    /// http://xml.org/sax/features/namespace-prefixes feature names.
    ///
    /// Implementors are free (and encouraged) to invent their own features, using names built on
    /// their own URIs.
    ///
    /// For documentation on Core Features, see
    /// https://svn.apache.org/repos/asf/xerces/xml-commons/tags/sax-2_0_1/java/external/xdocs/sax/features.html
    fn get_feature(&self, name: &str) -> Result<bool>;
    /// Look up the value of a property.
    ///
    /// The property name is any fully-qualified URI. It is possible for an XmlReader to recognize
    /// a property name but temporarily be unable to return its value. Some property values may be
    /// available only in specific contexts, such as before, during, or after a parse.
    ///
    /// XmlReaders are not required to recognize any specific property names, though an initial
    /// core set is documented for SAX2.
    ///
    /// Implementors are free (and encouraged) to invent their own properties, using names built on
    /// their own URIs.
    ///
    /// For documentation on Core Properties, see
    /// https://svn.apache.org/repos/asf/xerces/xml-commons/tags/sax-2_0_1/java/external/xdocs/sax/features.html
    // TODO: Add variant that takes/returns downcastable Boxed things?
    fn get_property_str(&self, name: &str) -> Result<String>;
    /// Parse an XML document.
    ///
    /// The application can use this method to instruct the XML reader to begin parsing an XML
    /// document from any valid input source (a character stream, a byte stream, or a URI).
    ///
    /// Applications may not invoke this method while a parse is in progress (they should create a
    /// new XmlReader instead for each nested XML document). Once a parse is complete, an
    /// application may reuse the same XmlReader object, possibly with a different input source.
    /// Configuration of the XmlReader object (such as handler bindings and values established for
    /// feature flags and properties) is unchanged by completion of a parse, unless the definition
    /// of that aspect of the configuration explicitly specifies other behavior. (For example,
    /// feature flags or properties exposing characteristics of the document being parsed.)
    ///
    /// During the parse, the XmlReader will provide information about the XML document through the
    /// registered event handlers.
    ///
    /// This method is synchronous: it will not return until parsing has ended. If a client
    /// application wants to terminate parsing early, it should throw an exception.
    fn parse(&self, input: &mut I) -> Result<()>;
    /// Parse an XML document from a system identifier (URI).
    ///
    /// This method is a shortcut for the common case of reading a document from a system
    /// identifier. It is the exact equivalent of the following:
    ///
    /// `parse(new InputSource(systemId));`
    ///
    /// If the system identifier is a URL, it must be fully resolved by the application before it
    /// is passed to the parser.
    fn parse_uri(&self, system_id: &str) -> Result<()>;
    /// Allow an application to register a content event handler.
    ///
    /// If the application does not register a content handler, all content events reported by the
    /// SAX parser will be silently ignored.
    ///
    /// Applications may register a new or different handler in the middle of a parse, and the SAX
    /// parser must begin using the new handler immediately.
    fn set_content_handler(&self, handler: CH);
    /// Allow an application to register a DTD event handler.
    ///
    /// If the application does not register a DTD handler, all DTD events reported by the SAX
    /// parser will be silently ignored.
    ///
    /// Applications may register a new or different handler in the middle of a parse, and the SAX
    /// parser must begin using the new handler immediately.
    fn set_dtd_handler(&self, handler: DH);
    /// Allow an application to register an entity resolver.
    ///
    /// If the application does not register an entity resolver, the XmlReader will perform its own
    /// default resolution.
    ///
    /// Applications may register a new or different resolver in the middle of a parse, and the SAX
    /// parser must begin using the new resolver immediately.
    fn set_entity_resolver(&self, resolver: ER);
    /// Allow an application to register an error event handler.
    ///
    /// If the application does not register an error handler, all error events reported by the SAX
    /// parser will be silently ignored; however, normal processing may not continue. It is highly
    /// recommended that all SAX applications implement an error handler to avoid unexpected bugs.
    ///
    /// Applications may register a new or different handler in the middle of a parse, and the SAX
    /// parser must begin using the new handler immediately.
    fn set_error_handler(&self, handler: EH);
    /// Set the value of a feature flag.
    ///
    /// The feature name is any fully-qualified URI. It is possible for an XmlReader to expose a
    /// feature value but to be unable to change the current value. Some feature values may be
    /// immutable or mutable only in specific contexts, such as before, during, or after a parse.
    ///
    /// All XmlReaders are required to support setting http://xml.org/sax/features/namespaces to
    /// true and http://xml.org/sax/features/namespace-prefixes to false.
    ///
    /// For documentation on Core Features, see
    /// https://svn.apache.org/repos/asf/xerces/xml-commons/tags/sax-2_0_1/java/external/xdocs/sax/features.html
    fn set_feature(&self, name: &str, value: bool) -> Result<()>;
    /// Set the value of a property.
    ///
    /// The property name is any fully-qualified URI. It is possible for an XmlReader to recognize
    /// a property name but to be unable to change the current value. Some property values may be
    /// immutable or mutable only in specific contexts, such as before, during, or after a parse.
    ///
    /// XmlReaders are not required to recognize setting any specific property names, though a core
    /// set is defined by SAX2.
    ///
    /// For documentation on Core Properties, see
    /// https://svn.apache.org/repos/asf/xerces/xml-commons/tags/sax-2_0_1/java/external/xdocs/sax/features.html
    // TODO: Add variant that takes/returns downcastable Boxed things?
    fn set_property_str(&self, name: &str, value: &str) -> Result<()>;
}
