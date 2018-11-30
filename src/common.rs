/// The SAX2 API definitions from which some of these interfaces were derived comes with
/// the following notice:
///
/// > This module, both source code and documentation, is in the Public Domain,
/// > and comes with NO WARRANTY. See http://www.saxproject.org for further
/// > information.
///
/// See also http://www.saxproject.org/copying.html

pub type Result<T> = std::result::Result<T, Box<Error>>;

pub trait Error: std::error::Error {}

/// A single input source for an XML entity.
///
/// This class allows a SAX application to encapsulate information about an input source in a
/// single object, which may include a public identifier, a system identifier, a byte stream
/// (possibly with a specified encoding), and/or a character stream.
///
/// There are two places that the application can deliver an input source to the parser: as the
/// argument to the `Parser.parse` method, or as the return value of the
/// `EntityResolver.resolve_entity` method.
///
/// The SAX parser will use the `InputSource` object to determine how to read XML input. If no
/// encoding is already set, the `InputSource` will read-ahead bytes until it can auotdetect a
/// character encoding using an algorithm such as the one in the XML specification.  If neither a
/// character stream nor a byte stream is available, the parser will attempt to open a URI
/// connection to the resource identified by the system identifier.
///
/// Modelled after `org.xml.sax.InputSource`
pub trait InputSource: std::io::Read {
    /// Create a new input source with a system identifier.
    ///
    /// Applications may use `set_public_id` to include a public identifier as well, or
    /// `set_encoding` to specify the character encoding, if known.
    ///
    /// If the system identifier is a URL, it must be fully resolved (it may not be a relative
    /// URL).
    fn new(system_id: &str) -> Self;
    /// Get the character encoding being used for the input source.
    fn get_encoding(&self) -> Option<String>;
    /// Get the public identifier for this input source.
    fn get_public_id(&self) -> Option<String>;
    /// Get the system identifier for this input source.
    ///
    /// The `get_encoding` method will return the character encoding of the object pointed to.
    ///
    /// If the system ID is a URL, it will be fully resolved.
    fn get_system_id(&self) -> Option<String>;
    /// Set the character encoding, if known.
    ///
    /// The encoding must be a string acceptable for an XML encoding declaration (see section 4.3.3
    /// of the XML 1.0 recommendation).
    fn set_encoding(&self, encoding: &str);
    /// Set the public identifier for this input source.
    ///
    /// The public identifier is always optional: if the application writer includes one, it will
    /// be provided as part of the location information.
    fn set_public_id(&self, public_id: &str);
    /// Set the system identifier for this input source.
    ///
    /// The system identifier is optional if there is a byte stream or a character stream, but it
    /// is still useful to provide one, since the application can use it to resolve relative URIs
    /// and can include it in error messages and warnings (the parser will attempt to open a
    /// connection to the URI only if there is no byte stream or character stream specified).
    ///
    /// If the application knows the character encoding of the object pointed to by the system
    /// identifier, it can register the encoding using the set_encoding method.
    ///
    /// If the system identifier is a URL, it must be fully resolved (it may not be a relative
    /// URL).
    fn set_system_id(&self, system_id: &str);
}

/// Interface for associating a SAX event with a document location.
/// If a SAX parser provides location information to the SAX application, it does so by implementing
/// this interface and then passing an instance to the application using the content handler's
/// `set_document_locator` method. The application can use the object to obtain the location of any
/// other SAX event in the XML source document.
///
/// Note that the results returned by the object will be valid only during the scope of each
/// callback method: the application will receive unpredictable results if it attempts to use the
/// locator at any other time, or after parsing completes.
///
/// SAX parsers are not required to supply a locator, but they are very strongly encouraged to do
/// so. If the parser supplies a locator, it must do so before reporting any other document events.
/// If no locator has been set by the time the application receives the `start_document` event, the
/// application should assume that a locator is not available.
///
/// Modelled after `org.xml.sax.Locator`
pub trait Locator {
    /// Return the column number where the current document event ends. This is one-based number of
    /// utf-8 char values since the last line end.
    ///
    /// Warning: The return value from the method is intended only as an approximation for the sake
    /// of diagnostics; it is not intended to provide sufficient information to edit the character
    /// content of the original XML document. For example, when lines contain combining character
    /// sequences, wide characters, surrogate pairs, or bi-directional text, the value may not
    /// correspond to the column in a text editor's display.
    ///
    /// The return value is an approximation of the column number in the document entity or external
    /// parsed entity where the markup triggering the event appears.
    ///
    /// If possible, the SAX driver should provide the line position of the first character after
    /// the text associated with the document event. The first column in each line is column 1.
    fn get_column_number(&self) -> Option<u64> {
        None
    }
    /// Return the line number where the current document event ends. Lines are delimited by line
    /// ends, which are defined in the XML specification.
    ///
    /// Warning: The return value from the method is intended only as an approximation for the sake
    /// of diagnostics; it is not intended to provide sufficient information to edit the character
    /// content of the original XML document. In some cases, these "line" numbers match what would
    /// be displayed as columns, and in others they may not match the source text due to internal
    /// entity expansion.
    ///
    /// The return value is an approximation of the line number in the document entity or external
    /// parsed entity where the markup triggering the event appears.
    ///
    /// If possible, the SAX driver should provide the line position of the first character after
    /// the text associated with the document event. The first line is line 1.
    fn get_line_number(&self) -> Option<u64> {
        None
    }
    /// Return the public identifier for the current document event.
    ///
    /// The return value is the public identifier of the document entity or of the external parsed
    /// entity in which the markup triggering the event appears.
    fn get_public_id(&self) -> Option<String> {
        None
    }
    /// Return the system identifier for the current document event.
    ///
    /// The return value is the system identifier of the document entity or of the external parsed
    /// entity in which the markup triggering the event appears.
    ///
    /// If the system identifier is a URL, the parser must resolve it fully before passing it to the
    /// application. For example, a file name must always be provided as a file:... URL, and other
    /// kinds of relative URI are also resolved against their bases.
    fn get_system_id(&self) -> Option<String> {
        None
    }
}

/// Basic interface for resolving entities.
///
/// If a SAX application needs to implement customized handling for external entities, it must
/// implement this interface and register an instance with the SAX driver using the
/// `set_entity_resolver` method.
///
/// The XML reader will then allow the application to intercept any external entities (including
/// the external DTD subset and external parameter entities, if any) before including them.
///
/// Many SAX applications will not need to implement this interface, but it will be especially
/// useful for applications that build XML documents from databases or other specialised input
/// sources, or for applications that use URI types other than URLs.
///
/// The following resolver would provide the application with a special character stream for the
/// entity with the system identifier "http://www.myhost.com/today":
///
/// use EntityResolver;
/// use InputSource;
///
/// ```
/// impl EntityResolve for MyResolve {
///   pub fn resolve_entity(&self, public_id: Option<&str>, system_id: &str) ->
///   Result<Option<Box<std::io::Read>>>
///   {
///     if systemd_id == "http://www.myhost.com/today" {
///       // return a special input source
///       let reader = MyReader::new();
///       return Ok(Some(Box::new(reader)));
///     } else {
///       // use the default behaviour
///       return None;
///     }
///   }
/// }
/// ```
///
/// The application can also use this interface to redirect system identifiers to local URIs or
/// to look up replacements in a catalog (possibly by using the public identifier).
///
/// Modelled after `org.xml.sax.EntityResolver`
pub trait EntityResolver {
    /// Resolve an external entity.
    ///
    /// Always return null, so that the parser will use the system identifier provided in the
    /// XML document. This method implements the SAX default behaviour: application writers can
    /// override it in a subclass to do special translations such as catalog lookups or URI
    /// redirection.
    #[allow(unused_variables)]
    fn resolve_entity(
        &self,
        public_id: Option<&str>,
        system_id: &str,
    ) -> Result<Option<Box<std::io::Read>>> {
        Ok(None)
    }
}
