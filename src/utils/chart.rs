use std::path::Path;

use plotters::prelude::*;

use crate::report::Report;

#[derive(Debug, Clone)]
pub struct ChartConfig {
    pub width: u32,
    pub height: u32,
    pub top_n: u32,
}

impl Default for ChartConfig {
    fn default() -> Self {
        Self {
            width: 1024,
            height: 798,
            top_n: 10,
        }
    }
}

pub struct ChartDrawer<'a> {
    config: ChartConfig,
    report: &'a Report,
}

impl<'a> ChartDrawer<'a> {
    pub fn new(report: &'a Report, config: Option<ChartConfig>) -> Self {
        let config = config.unwrap_or_default();
        
        ChartDrawer {
            config,
            report,
        }
    }

    fn get_sorted(&self) -> Vec<(String, usize)> {

        let sorted = self.report.sort_stats(|&a, &b| {
            a.1.lines.cmp(&b.1.lines)
        });

        let mut total = sorted.iter()
            .map(|(a, b)| {
            (a.to_string(), b.lines)
            })
            .collect::<Vec<_>>();

        let top_n = self.config.top_n as usize;

        if total.len() > top_n {
            let other = total.split_off(top_n);
            total.truncate(top_n);
            
            let other = other.iter().map(|(_, stat)| {
                stat
            })
            .sum::<usize>();

            total.push(("Other".to_string(), other));
        }

        total
    }

    pub fn draw_pie<P: AsRef<Path>>(&self, path: P) -> Result<(), Box<dyn std::error::Error>> { 
        // 1. 创建位图后端和绘图区域
        let root = BitMapBackend::new(&path, (self.config.width, self.config.height))
            .into_drawing_area();
        root.fill(&WHITE)?;

        // 2. 绘制标题
        let title_style = TextStyle::from(("sans-serif", 30)).color(&BLACK);
        root.draw_text(
            "Project Code Distribution (Lines of Code)",
            &title_style,
            (20, 20),
        )?;

        // 3. 获取排序后的统计数据
        let stat = self.get_sorted();
        let total_lines: f64 = stat.iter().map(|(_, lines)| *lines as f64).sum();
        if total_lines == 0.0 {
            return Err("Total lines of code is zero, cannot draw pie chart".into());
        }

        // 4. 计算饼图的中心坐标和半径
        let pie_radius = (std::cmp::min(self.config.width, self.config.height) as f64 / 2.5) - 50.0;
        let center = (
            self.config.width as i32 / 2,
            self.config.height as i32 / 2 + 30, // 向下偏移避免和标题重叠
        );

        // 5. 定义颜色序列（支持自动循环，适配更多分类）
        let color_sequence = [
            &RGBColor(255, 99, 132),
            &RGBColor(54, 162, 235),
            &RGBColor(255, 206, 86),
            &RGBColor(75, 192, 192),
            &RGBColor(153, 102, 255),
            &RGBColor(255, 159, 64),
            &RGBColor(231, 233, 237),
        ];

        unimplemented!()
    }
}