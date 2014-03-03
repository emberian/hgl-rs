use gl;
use gl::types::{GLuint, GLenum};

pub enum QueryTarget {
    SamplesPassed,
}

impl QueryTarget {
    fn to_glenum(&self) -> GLenum {
        match *self {
            SamplesPassed => gl::SAMPLES_PASSED,
        }
    }
}

pub struct Query {
    name: GLuint,
    target: GLenum,
}

impl Query {
    pub fn new(target: QueryTarget) -> Query {
        let mut name = 0;

        unsafe {
            gl::GenQueries(1, &mut name);
        }

        Query {
            name: name,
            target: target.to_glenum(),
        }
    }

    pub fn begin(&self) {
        gl::BeginQuery(self.target, self.name);
    }

    pub fn end(&self) {
        gl::EndQuery(self.target);
    }

    pub fn result_available(&self) -> bool {
        let mut params = 0;
        unsafe {
            gl::GetQueryObjectuiv(self.name, gl::QUERY_RESULT_AVAILABLE, &mut params);
        }
        params == gl::TRUE as u32
    }

    pub fn result(&self) -> GLuint {
        let mut params = 0;
        unsafe {
            gl::GetQueryObjectuiv(self.name, gl::QUERY_RESULT, &mut params);
        }
        params
    }
}

impl Drop for Query {
    fn drop(&mut self) {
        unsafe {
            gl::DeleteQueries(1, &self.name);
        }
    }
}

