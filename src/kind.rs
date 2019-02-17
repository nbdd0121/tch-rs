#[derive(Debug)]
pub enum Kind {
    Uint8,
    Int8,
    Int16,
    Int,
    Int64,
    Half,
    Float,
    Double,
    ComplexHalf,
    ComplexFloat,
    ComplexDouble,
}

impl Kind {
    pub(crate) fn c_int(&self) -> libc::c_int {
        match self {
            Kind::Uint8 => 0,
            Kind::Int8 => 1,
            Kind::Int16 => 2,
            Kind::Int => 3,
            Kind::Int64 => 4,
            Kind::Half => 5,
            Kind::Float => 6,
            Kind::Double => 7,
            Kind::ComplexHalf => 8,
            Kind::ComplexFloat => 9,
            Kind::ComplexDouble => 10,
        }
    }

    pub(crate) fn of_c_int(v: libc::c_int) -> Kind {
        match v {
            0 => Kind::Uint8,
            1 => Kind::Int8,
            2 => Kind::Int16,
            3 => Kind::Int,
            4 => Kind::Int64,
            5 => Kind::Half,
            6 => Kind::Float,
            7 => Kind::Double,
            8 => Kind::ComplexHalf,
            9 => Kind::ComplexFloat,
            10 => Kind::ComplexDouble,
            _ => panic!("unexpected kind {}", v),
        }
    }
}