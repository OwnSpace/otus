/// Умная розетка
pub struct SmartSocket {
    /// Потребляемая мощность
    pub power_consumption: f32,

    /// Текущее состояние
    pub state: SmartSocketState,
}

impl SmartSocket {
    /// Текстовое описание
    pub fn description(&self) -> String {
        todo!()
    }

    /// Включить/выключить розетку
    pub fn toggle(&self) -> Result<SmartSocketState, String> {
        todo!()
    }
}

/// Состояние розетки
pub enum SmartSocketState {
    /// Включена
    On,

    /// Выключена
    Off,
}

/// Температура
pub struct Temperature(f32);

/// Термометр
pub struct Thermometer {
    /// Температура в градусах Цельсия
    pub temperature_celsius: Temperature,
}

impl Thermometer {
    pub fn current_temperature(&self) -> Temperature {
        todo!()
    }
}
