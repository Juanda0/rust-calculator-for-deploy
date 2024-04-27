pub struct Calculadora;

impl Calculadora {
    pub fn cuadrado(numero: f64) -> f64 {
        numero * numero
    }
    
    pub fn cubo(numero: f64) -> f64 {
        numero * numero * numero
    }
}