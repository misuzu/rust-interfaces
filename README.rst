Rust bindings to retrieve network interface information
======================================================================

:Date: 5/9 2021
:RustVersion: stable


.. contents::


OS Support
-------------

*   Windows
*   Unix-Like( BSD-Like, XNU, Linux )


Run
-------

.. code:: bash
    
    git clone https://github.com/zkonge/rust-ifaces.git
    cargo run --example ifaces


Example
-----------

.. code:: toml
    
    [dependencies]
    ifaces = { git = "https://github.com/zkonge/rust-ifaces.git" }


.. code:: rust
    
    fn main() {
        match ifaces::ifaces() {
            Ok(interfaces) => {
                for interface in interfaces.into_iter() {
                    println!("Found Interface: {:?}", interface)
                }
            },
            Err(_) => println!("Ooops ...")
        };
    }



Thanks
---------

*   `dlevy47 <https://github.com/dlevy47/rust-interfaces>`_ , Origin code(linux platform)
*   `GGist <https://github.com/GGist/rust-ifaces>`_ , windows platform code
