use crate::Calculadora;

#[cfg(test)]
mod tests {
    use crate::Calculadora;

    #[test]
    fn test_cuadrado_positivo() {
        assert_eq!(25.0, Calculadora::cuadrado(5.0));
    }

    #[test]
    fn test_cuadrado_cero() {
        assert_eq!(0.0, Calculadora::cuadrado(0.0));
    }

    #[test]
    fn test_cuadrado_uno() {
        assert_eq!(1.0, Calculadora::cuadrado(1.0));
    }

    #[test]
    fn test_cuadrado_negativo() {
        assert_eq!(16.0, Calculadora::cuadrado(-4.0));
    }

    #[test]
    fn test_cubo_positivo() {
        assert_eq!(125.0, Calculadora::cubo(5.0));
    }

    #[test]
    fn test_cubo_cero() {
        assert_eq!(0.0, Calculadora::cubo(0.0));
    }

    #[test]
    fn test_cubo_uno() {
        assert_eq!(1.0, Calculadora::cubo(1.0));
    }

    #[test]
    fn test_cubo_menos_uno() {
        assert_eq!(-1.0, Calculadora::cubo(-1.0));
    }

    #[test]
    fn test_cubo_negativo() {
        assert_eq!(-64.0, Calculadora::cubo(-4.0));
    }
}
