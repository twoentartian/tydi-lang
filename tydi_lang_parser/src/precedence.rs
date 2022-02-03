use pest::{Parser, RuleType};
use pest::iterators::{Pairs};
use pest::prec_climber::{PrecClimber};
use pest::prec_climber::Assoc::{Left, Right};
use pest::prec_climber as pcl;

#[derive(Parser)]
#[grammar = "tydi_lang_syntax.pest"]
pub struct TydiParser;

lazy_static! {
    static ref PREC_CLIMBER: PrecClimber<Rule> = {

        PrecClimber::new(vec![
            pcl::Operator::new(Rule::OP_LeftShift, Left) | pcl::Operator::new(Rule::OP_LeftShift, Left),
            pcl::Operator::new(Rule::OP_Add, Left) | pcl::Operator::new(Rule::OP_Minus, Left),
        ])
    };
}
