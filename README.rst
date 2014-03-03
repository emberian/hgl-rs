hgl-rs
======

.. image:: https://travis-ci.org/cmr/hgl-rs.png?branch=master
           :target: https://travis-ci.org/cmr/hgl-rs

Documentation_ is hosted by rust-ci_

``hgl-rs``, short for "gl helpers for Rust", provides some boilerplate for
working with OpenGL. I wrote it as part of my `graphics
course`_ at Clarkson_.

.. note::
    ``hgl-rs`` exposes a safe interface, but it is still trivial to create
    invalid state that would cause GL to be memory unsafe. ``hgl-rs`` does not
    attempt to provide an actual safe wrapper for GL, but rather a convenient
    Rusty wrapper around the concepts GL exposes.

license
-------

``hgl-rs`` is available under the same license as Rust (dual Apache2/MIT)

.. _`graphics course`: http://web2.clarkson.edu/class/cs452/
.. _Clarkson: http://clarkson.edu/
.. _Documentation: http://rust-ci.org/cmr/hgl-rs/doc/hgl/
.. _rust-ci: http://rust-ci.org/
