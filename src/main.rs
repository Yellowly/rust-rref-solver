use std::{collections::VecDeque, io::BufRead, fmt::{Display, Formatter, Result}, clone};

use yew::prelude::*;
use web_sys::{HtmlInputElement, TouchEvent, TouchList, Touch, Element, Event, window, HtmlDocument};
use wasm_bindgen::{JsCast, JsValue, UnwrapThrowExt};
use gloo::{console::{self, Timer, dirxml}, timers::callback, events::EventListener, utils::document};
use gloo::timers::callback::{Interval, Timeout};

mod rref;

fn main() {
    yew::start_app::<RootComponent>();
}


enum Msg{
    SetDim((u32,u32)),
    SetMatrixVal((u32,u32),String),
    Solve,
    Revert,
    None,
}
struct RootComponent{
    dimens: (u32,u32),
    matrix: Vec<Vec<String>>,
    prev_matrix: Vec<Vec<String>>
}
impl Component for RootComponent{
    type Message = Msg;

    type Properties = ();

    fn create(ctx: &Context<Self>) -> Self {

        Self{dimens: (0,0),matrix: vec![vec![String::default();0];0], prev_matrix: vec![vec![String::default();0];0]}
    }
    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::SetDim((rows,cols)) => {
                let r: u32 = rows;
                let c: u32 = cols;
                self.dimens=(r,c);
                self.matrix = vec![vec![String::default();self.dimens.1 as usize];self.dimens.0 as usize];
                true
            }
            Msg::SetMatrixVal((r,c), v) => {
                self.matrix[r as usize][c as usize]=v;
                true
            }
            Msg::Solve => {
                self.prev_matrix=self.matrix.clone();
                let m: Vec<Vec<f64>> = self.matrix.iter().map(|v: &Vec<String>| v.iter().map(|s| s.parse::<f64>().unwrap_or(0.0)).collect()).collect();
                self.matrix = rref::convert(m).iter().map(|v: &Vec<f64>| v.iter().map(|f| round(f).to_string()).collect()).collect();
                true
            }
            Msg::Revert => {
                self.dimens=(self.prev_matrix.len() as u32,self.prev_matrix.get(0).unwrap_or(&vec![String::default();1]).len() as u32);
                let temp=self.prev_matrix.clone();
                self.prev_matrix=self.matrix.clone();
                self.matrix=temp;
                true
            }
            Msg::None => {
                true
            }
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let link = ctx.link();
        let rows = self.dimens.0;
        let cols = self.dimens.1;
        html!{
            <div class="content">
                <div class="dimentions">
                    <input type="number" name="rows" min="0" max="9" oninput={link.callback(move |event: InputEvent| {let input: HtmlInputElement = event.target_unchecked_into(); Msg::SetDim((input.value().parse::<u32>().unwrap_or(1), cols))})} onfocus={link.callback(|e: FocusEvent| {let i: HtmlInputElement = e.target_unchecked_into(); i.select(); Msg::None})}/>
                    <div class="x">{"x"}</div>
                    <input type="number" name="cols" min="0" max="9" oninput={link.callback(move |event: InputEvent| {let input: HtmlInputElement = event.target_unchecked_into(); Msg::SetDim((rows,input.value().parse::<u32>().unwrap_or(1)))})} onfocus={link.callback(|e: FocusEvent| {let i: HtmlInputElement = e.target_unchecked_into(); i.select(); Msg::None})}/>
                </div>
                <div class="matrix">
                {
                    (0..self.dimens.0).map(|r: u32|{
                        html!{
                            <div class="row">
                            {(0..self.dimens.1).map(|c: u32| {
                                html!{
                                    <div class="matrix-input">
                                    <input value={self.matrix[r as usize][c as usize].clone()} onfocus={link.callback(|e: FocusEvent| {let i: HtmlInputElement = e.target_unchecked_into(); i.select(); Msg::None})} oninput={link.callback(move |event: InputEvent| {let input: HtmlInputElement = event.target_unchecked_into(); Msg::SetMatrixVal((r.clone(),c.clone()),input.value())})}/>
                                    </div>
                                }
                            }).collect::<Html>()
                            }</div>
                    }
                    }).collect::<Html>()
                }
                </div>
                <button onclick={link.callback(|_| Msg::Solve)}> {"solve"}</button>
                if self.prev_matrix.len()!=0{
                    <button onclick={link.callback(|_| Msg::Revert)}> {"revert"}</button>
                }
            </div>
        }
    }
}

fn round(v: &f64) -> f64{
    return (v * 100_000_000.0).round() / 100_000_000.0;

}