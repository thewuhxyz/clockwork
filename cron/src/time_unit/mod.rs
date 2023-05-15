mod days_of_month;
mod days_of_week;
mod hours;
mod minutes;
mod months;
mod seconds;
mod years;

pub use self::days_of_month::DaysOfMonth;
pub use self::days_of_week::DaysOfWeek;
pub use self::hours::Hours;
pub use self::minutes::Minutes;
pub use self::months::Months;
pub use self::seconds::Seconds;
pub use self::years::Years;

use crate::error::*;
use crate::ordinal::{Ordinal, OrdinalSet};
use crate::specifier::{RootSpecifier, Specifier};
use std::borrow::Cow;
use std::iter;

/// Methods exposing a schedule's configured ordinals for each individual unit of time.
/// # Example
/// ```
/// use clockwork_cron::{Schedule,TimeUnitSpec};
/// use std::ops::Bound::{Included,Excluded};
/// use std::str::FromStr;
///
/// let expression = "* * * * * * 2015-2044";
/// let schedule = Schedule::from_str(expression).expect("Failed to parse expression.");
///
/// // Membership
/// assert_eq!(true, schedule.years().includes(2031));
/// assert_eq!(false, schedule.years().includes(1969));
///
/// // Number of years specified
/// assert_eq!(30, schedule.years().count());
/// ```
pub trait TimeUnitSpec {
    /// Returns true if the provided ordinal was included in the schedule spec for the unit of time
    /// being described.
    /// # Example
    /// ```
    /// use clockwork_cron::{Schedule,TimeUnitSpec};
    /// use std::str::FromStr;
    ///
    /// let expression = "* * * * * * 2015-2044";
    /// let schedule = Schedule::from_str(expression).expect("Failed to parse expression.");
    ///
    /// // Membership
    /// assert_eq!(true, schedule.years().includes(2031));
    /// assert_eq!(false, schedule.years().includes(2004));
    /// ```
    fn includes(&self, ordinal: Ordinal) -> bool;

    /// Returns the number of ordinals included in the associated schedule
    /// # Example
    /// ```
    /// use clockwork_cron::{Schedule,TimeUnitSpec};
    /// use std::str::FromStr;
    ///
    /// let expression = "* * * 1,15 * * *";
    /// let schedule = Schedule::from_str(expression).expect("Failed to parse expression.");
    ///
    /// assert_eq!(2, schedule.days_of_month().count());
    /// ```
    fn count(&self) -> u32;

    /// Checks if this TimeUnitSpec is defined as all possibilities (thus created with a '*', '?' or in the case of weekdays '1-7')
    /// # Example
    /// ```
    /// use clockwork_cron::{Schedule,TimeUnitSpec};
    /// use std::str::FromStr;
    ///
    /// let expression = "* * * 1,15 * * *";
    /// let schedule = Schedule::from_str(expression).expect("Failed to parse expression.");
    ///
    /// assert_eq!(false, schedule.days_of_month().is_all());
    /// assert_eq!(true, schedule.months().is_all());
    /// ```
    fn is_all(&self) -> bool;
}

impl<T> TimeUnitSpec for T
where
    T: TimeUnitField,
{
    fn includes(&self, ordinal: Ordinal) -> bool {
        self.ordinals().contains(&ordinal)
    }

    fn count(&self) -> u32 {
        self.ordinals().len() as u32
    }

    fn is_all(&self) -> bool {
        let max_supported_ordinals = Self::inclusive_max() - Self::inclusive_min() + 1;
        self.ordinals().len() == max_supported_ordinals as usize
    }
}

pub trait TimeUnitField
where
    Self: Sized,
{
    fn from_optional_ordinal_set(
        ordinal_set: Option<OrdinalSet>,
        field: Vec<RootSpecifier>,
    ) -> Self;
    fn name() -> Cow<'static, str>;
    fn name_in_text() -> Cow<'static, str>;
    fn inclusive_min() -> Ordinal;
    fn inclusive_max() -> Ordinal;
    fn ordinals(&self) -> OrdinalSet;
    fn to_human_text(&self) ->  Result<String, Error>;

    fn from_ordinal(ordinal: Ordinal) -> Self {
        Self::from_ordinal_set(
            iter::once(ordinal).collect(),
            vec![RootSpecifier::from(Specifier::Point(ordinal))],
        )
    }

    fn supported_ordinals() -> OrdinalSet {
        (Self::inclusive_min()..Self::inclusive_max() + 1).collect()
    }

    fn all() -> Self {
        Self::from_optional_ordinal_set(None, vec![RootSpecifier::from(Specifier::All)])
    }

    fn from_ordinal_set(ordinal_set: OrdinalSet, field: Vec<RootSpecifier>) -> Self {
        Self::from_optional_ordinal_set(Some(ordinal_set), field)
    }

    fn ordinal_from_name(name: &str) -> Result<Ordinal, Error> {
        Err(ErrorKind::Expression(format!(
            "The '{}' field does not support using names. '{}' \
             specified.",
            Self::name(),
            name
        ))
        .into())
    }

    fn name_from_ordinal(ordinal: Ordinal) -> Result<String, Error> {
        Ok(ordinal.to_string())
    }

    fn validate_ordinal(ordinal: Ordinal) -> Result<Ordinal, Error> {
        //println!("validate_ordinal for {} => {}", Self::name(), ordinal);
        match ordinal {
            i if i < Self::inclusive_min() => Err(ErrorKind::Expression(format!(
                "{} must be greater than or equal to {}. ('{}' \
                 specified.)",
                Self::name(),
                Self::inclusive_min(),
                i
            ))
            .into()),
            i if i > Self::inclusive_max() => Err(ErrorKind::Expression(format!(
                "{} must be less than {}. ('{}' specified.)",
                Self::name(),
                Self::inclusive_max(),
                i
            ))
            .into()),
            i => Ok(i),
        }
    }

    fn ordinals_from_specifier(specifier: &Specifier) -> Result<OrdinalSet, Error> {
        use self::Specifier::*;
        //println!("ordinals_from_specifier for {} => {:?}", Self::name(), specifier);
        match *specifier {
            All => Ok(Self::supported_ordinals().clone()),
            Point(ordinal) => Ok((&[ordinal]).iter().cloned().collect()),
            Range(start, end) => {
                match (Self::validate_ordinal(start), Self::validate_ordinal(end)) {
                    (Ok(start), Ok(end)) if start <= end => Ok((start..end + 1).collect()),
                    _ => Err(ErrorKind::Expression(format!(
                        "Invalid range for {}: {}-{}",
                        Self::name(),
                        start,
                        end
                    ))
                    .into()),
                }
            }
            NamedRange(ref start_name, ref end_name) => {
                let start = Self::ordinal_from_name(start_name)?;
                let end = Self::ordinal_from_name(end_name)?;
                match (Self::validate_ordinal(start), Self::validate_ordinal(end)) {
                    (Ok(start), Ok(end)) if start <= end => Ok((start..end + 1).collect()),
                    _ => Err(ErrorKind::Expression(format!(
                        "Invalid named range for {}: {}-{}",
                        Self::name(),
                        start_name,
                        end_name
                    ))
                    .into()),
                }
            }
        }
    }

    fn ordinals_from_root_specifier(root_specifier: &RootSpecifier) -> Result<OrdinalSet, Error> {
        let ordinals = match root_specifier {
            RootSpecifier::Specifier(specifier) => Self::ordinals_from_specifier(specifier)?,
            RootSpecifier::Period(start, step) => {
                let base_set = match start {
                    // A point prior to a period implies a range whose start is the specified
                    // point and terminating inclusively with the inclusive max
                    Specifier::Point(start) => {
                        let start = Self::validate_ordinal(*start)?;
                        (start..=Self::inclusive_max()).collect()
                    }
                    specifier => Self::ordinals_from_specifier(specifier)?,
                };
                if *step == 0 {
                    return Err(ErrorKind::Expression(format!("step cannot be 0")).into());
                }
                base_set.into_iter().step_by(*step as usize).collect()
            }
            RootSpecifier::NamedPoint(ref name) => (&[Self::ordinal_from_name(name)?])
                .iter()
                .cloned()
                .collect::<OrdinalSet>(),
        };
        Ok(ordinals)
    }

    fn human_text_from_specifier(specifier: &Specifier) -> Result<String, Error> {
        use self::Specifier::*;
        //println!("ordinals_from_specifier for {} => {:?}", Self::name(), specifier);
        match *specifier {
            All => Ok(format!("")),
            Point(ordinal) => Ok(format!("{}", &Self::name_from_ordinal(ordinal)?)),
            Range(start, end) => {
                match (Self::validate_ordinal(start), Self::validate_ordinal(end)) {
                    (Ok(start), Ok(end)) if start <= end => Ok(format!(
                        "from {} through {}",
                        &Self::name_from_ordinal(start)?,
                        &Self::name_from_ordinal(end)?
                    )),
                    _ => Err(ErrorKind::Expression(format!(
                        "Invalid range for {}: {}-{}",
                        Self::name(),
                        start,
                        end
                    ))
                    .into()),
                }
            }
            NamedRange(ref start_name, ref end_name) => {
                let start = Self::ordinal_from_name(start_name)?;
                let end = Self::ordinal_from_name(end_name)?;
                match (Self::validate_ordinal(start), Self::validate_ordinal(end)) {
                    (Ok(start), Ok(end)) if start <= end => Ok(format!(
                        "from {} through {}",
                        &Self::name_from_ordinal(start)?,
                        &Self::name_from_ordinal(end)?
                    )),
                    _ => Err(ErrorKind::Expression(format!(
                        "Invalid named range for {}: {}-{}",
                        Self::name(),
                        start_name,
                        end_name
                    ))
                    .into()),
                }
            }
        }
    }

    fn human_text_from_root_specifier(root_specifier: &RootSpecifier) -> Result<String, Error> {
        let ordinals = match root_specifier {
            RootSpecifier::Specifier(specifier) => match specifier {
                Specifier::Point(_) => format!("{}", Self::human_text_from_specifier(specifier)?),
                _ => format!("every {} {}", Self::name_in_text(), Self::human_text_from_specifier(specifier)?),
            },
            RootSpecifier::Period(specifier, step) => {
                if *step == 0 {
                    return Err(ErrorKind::Expression(format!("step cannot be 0")).into());
                }
                match specifier {
                    // A point prior to a period implies a range whose start is the specified
                    // point and terminating inclusively with the inclusive max
                    Specifier::Point(start) => {
                        let start = Self::validate_ordinal(*start)?;
                        format!(
                            "every {} {} from {} through {}",
                            &Self::suffix(*step),
                            &Self::name_in_text(),
                            &Self::name_from_ordinal(start)?,
                            &Self::inclusive_max()
                        )
                    }
                    specifier => format!("every {} {}", &Self::suffix(*step), &Self::human_text_from_specifier(specifier)?),
                }
            }
            RootSpecifier::NamedPoint(ref name) => {
                // validates name
                let ordinal = Self::ordinal_from_name(name)?;
                let name = Self::name_from_ordinal(ordinal)?;
                format!("{name}")
            }
        };
        Ok(ordinals)
    }

    fn human_text_from_field(field: Vec<RootSpecifier>, named: bool) -> Result<String, Error> {
        let mut human_text_list_from_field: Vec<String> = vec![];
        for root_specifier in field {
            human_text_list_from_field.push(Self::human_text_from_root_specifier(&root_specifier)?)
        }

        let human_text_string = match human_text_list_from_field.len() {
            0 => format!(""),
            1 => format!("{}", human_text_list_from_field[0].to_string()),
            2 => format!(
                "{} and {}",
                &human_text_list_from_field[0], &human_text_list_from_field[1]
            ),
            _ => {
                format!(
                    "{}, and {}",
                    &human_text_list_from_field[0..human_text_list_from_field.len() - 1].join(", "),
                    &human_text_list_from_field[human_text_list_from_field.len() - 1]
                )
            }
        };
        let s = match named {
            true => "".to_owned(),
            false => format!("{} ", &Self::name_in_text()),
        };

        Ok(format!("{}{}", &s, &human_text_string)
            .replace("every 1st", "every")
            .replace(&format!("{} every", &Self::name_in_text()), "every")
            .replace(&format!(", {}", &Self::name_in_text()), ",")
            .replace(&format!(", and {}", &Self::name_in_text()), ", and"))
    }

    fn suffix(num: Ordinal) -> String {
        
            match num % 10 {
                1 => num.to_string() + "st",
                2 => num.to_string() + "nd",
                3 => num.to_string() + "rd",
                _ => num.to_string() + "th",
      
        }
    }
}
