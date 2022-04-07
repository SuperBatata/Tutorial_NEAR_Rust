pub mod temp_format;


use near_sdk::{
    borsh::{ self, BorshDeserialize, BorshSerialize },
    serde::{ Deserialize, Serialize },
};


use temp_format::TempFormat;


#[derive(BorshDeserialize, BorshSerialize, Serialize, Deserialize)]
#[serde(crate = "near_sdk::serde")]
pub struct Temperature {
    value: f32,
    temp_format: String,
}


impl Temperature {

    /// Cria uma instância de temperatura. O formato será definido pelo formato de sistema "temp_format".
    /// 
    /// "arg_temp" é o formato de temperatura da mensagem recebida.
    /// 
    /// Se arg_temp for diferente de temp_format. Seguirá estes passos:
    ///  - Cria uma instância de temperatura no formato de arg_temp;
    ///  - Converte seu formato para temp_format;
    ///  - Retorna a temperatura;
    /// 
    pub fn new(value: f32, temp_format: &TempFormat, arg_temp: Option<String>) -> Self {
        match arg_temp {
            None => {
                // Argumento não descreve temperatura. Portanto, utiliza a definida no sistema.
                let temp_format: String = format!("{}", temp_format);

                Temperature{
                    value,
                    temp_format,
                }
            },
            Some(arg_string) => {
                // Argumento descreve temperatura. Portanto, cria uma instância com essa temperatura.
                let arg_format = String::from(
                    TempFormat::from(arg_string)
                );

                let mut arg_temp = Temperature{
                    value,
                    temp_format: arg_format,
                };

                // Converte a temperatura da mensagem para a temperatura do sistema.
                arg_temp.convert(temp_format);
                arg_temp
            }
        }

    }
    
    /// Convert this type to another.
    pub fn convert(&mut self, temp_format: &TempFormat){
        let current: TempFormat = TempFormat::from(&self.temp_format);

        let value = self.value;

        match (current, temp_format) {
            (TempFormat::Kelvin, &TempFormat::Celsius) => {
                // C = K − 273.15
                self.value = value - 273.15;    
                // self.temp_format = TempFormat::Celsius;
                self.temp_format = String::from(TempFormat::Celsius);
            },
            (TempFormat::Kelvin, &TempFormat::Fahrenheit) => {
                // F = (K – 273.15) × 9⁄5 + 32
                self.value = (value - 273.15) * 9.0 / 5.0 + 32.0;
                // self.temp_format = TempFormat::Fahrenheit;
                self.temp_format = String::from(TempFormat::Fahrenheit);
            },
            (TempFormat::Celsius, &TempFormat::Kelvin) => {
                // K = C + 273.15
                self.value = value + 273.15;
                // self.temp_format = TempFormat::Kelvin;
                self.temp_format = String::from(TempFormat::Kelvin);
            },
            (TempFormat::Celsius, &TempFormat::Fahrenheit) => {
                // F = C(9⁄5) + 32
                self.value = value * (9.0 / 5.0) + 32.0;
                // self.temp_format = TempFormat::Fahrenheit;
                self.temp_format = String::from(TempFormat::Fahrenheit);
            },
            (TempFormat::Fahrenheit, &TempFormat::Kelvin) => {
                // K = (F − 32) × 5⁄9 + 273.15
                self.value = (value - 32.0) * 5.0 / 9.0 + 273.15;
                // self.temp_format = TempFormat::Kelvin;
                self.temp_format = String::from(TempFormat::Kelvin);
            },
            (TempFormat::Fahrenheit, &TempFormat::Celsius) => {
                // C = (F − 32) × 5⁄9
                self.value = (value - 32.0) * 5.0 / 9.0;
                // self.temp_format = TempFormat::Celsius;
                self.temp_format = String::from(TempFormat::Celsius);
            },
            (_, _) => {
                return;
            }
        }
    }
}
