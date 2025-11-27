//! # Budget - Sistema de prevenção de monopolização de CPU

/// Sistema de budget para evitar que tarefas monopolizem CPU.
///
/// Cada tarefa recebe um budget de execuções antes de yield voluntário.
pub struct Budget {
    remaining: usize,
}

impl Budget {
    /// Cria novo budget com quantidade inicial.
    ///
    /// # Argumentos
    ///
    /// * `initial` - Número de operações permitidas
    pub fn new(initial: usize) -> Self {
        Self { remaining: initial }
    }

    /// Consome 1 unidade de budget.
    ///
    /// # Retorna
    ///
    /// `true` se ainda há budget, `false` se esgotado.
    pub fn consume(&mut self) -> bool {
        if self.remaining > 0 {
            self.remaining -= 1;
            true
        } else {
            false
        }
    }

    /// Reseta budget para novo valor.
    ///
    /// # Argumentos
    ///
    /// * `amount` - Nova quantidade de budget
    pub fn reset(&mut self, amount: usize) {
        self.remaining = amount;
    }
}
