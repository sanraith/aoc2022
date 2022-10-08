// mod solutions {
//     trait SolutionBase {
//         fn init(&self);
//     }

//     trait Solution1 {
//         // // fn init();

//         // fn get_common_mut(&mut self) -> &mut Common;
//         // fn init(&mut self) -> () {
//         //     let a = self.get_common_mut();
//         //     a.input = "".to_owned();
//         // }
//         fn part1(&self, input: &str) -> Result<String, String>;
//         fn part2(&self, input: &str) -> Result<String, String>;
//     }

//     // struct Common {
//     //     // input: String,
//     // }

//     struct Day01 {
//         // common: Common,
//     }
//     impl Solution1 for Day01 {
//         fn part1(&self, input: &str) -> Result<String, String> {
//             Ok(input.to_owned())
//         }

//         fn part2(&self, input: &str) -> Result<String, String> {
//             Ok(input.to_owned())
//         }

//         // fn get_common_mut(&mut self) -> &mut Common {
//         //     &mut self.common
//         // }
//     }

//     trait SolutionV2<'a> {
//         fn new(context: ContextV1<'a>) -> Self;
//         fn part1(&self) -> Result<String, String>;
//         fn part2(&self) -> Result<String, String>;
//     }

//     pub struct ContextV1<'a> {
//         pub input: &'a str,
//     }

//     pub struct Day02<'a> {
//         context: ContextV1<'a>,
//     }

//     impl<'a> SolutionV2<'a> for Day02<'a> {
//         fn new(context: ContextV1<'a>) -> Day02<'a> {
//             Day02 { context }
//         }

//         fn part1(&self) -> Result<String, String> {
//             todo!()
//         }

//         fn part2(&self) -> Result<String, String> {
//             todo!()
//         }
//     }

//     struct ContextV2;
//     impl ContextV2 {
//         fn progress(value: f32) {}
//     }

//     trait SolutionV3 {
//         fn part1(&self, input: &str, ctx: &ContextV2) -> Result<String, String>;
//         fn part2(&self, input: &str, ctx: &ContextV2) -> Result<String, String>;
//     }

//     struct Day03;
//     impl SolutionV3 for Day03 {
//         fn part1(&self, input: &str, context: &ContextV2) -> Result<String, String> {
//             todo!()
//         }

//         fn part2(&self, input: &str, context: &ContextV2) -> Result<String, String> {
//             todo!()
//         }
//     }

//     struct ContextV3<'a> {
//         input: &'a str,
//     }
//     impl<'a> ContextV3<'a> {
//         fn progress(value: f32) {}
//     }

//     trait SolutionV4 {
//         fn part1(&self, ctx: &ContextV3) -> Result<String, String>;
//         fn part2(&self, ctx: &ContextV3) -> Result<String, String>;
//     }

//     struct Day04;
//     impl SolutionV4 for Day04 {
//         fn part1(&self, ctx: &ContextV3) -> Result<String, String> {
//             todo!()
//         }

//         fn part2(&self, ctx: &ContextV3) -> Result<String, String> {
//             todo!()
//         }
//     }
// }

// mod test2 {

//     trait CreateNew<'a> {
//         fn new(data: &'a str) -> Self;
//     }

//     struct Item<'b> {
//         data: &'b str,
//     }

//     impl<'c> CreateNew<'c> for Item<'c> {
//         fn new(data: &'c str) -> Self {
//             Item { data }
//         }
//     }
// }
