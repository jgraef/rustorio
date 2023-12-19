use rustorio_prototype::Id;
use yew::{
    function_component,
    html,
    Html,
};

use crate::ui::select::{
    DefaultSelected,
    LabSelect,
    TechnologySelect,
};

#[function_component]
pub fn Research() -> Html {
    html! {
        <form class="w-100">
            <div class="card w-75 mx-auto mb-3">
                <div class="card-header">
                    {"Lab Setup"}
                </div>
                <div class="card-body">
                    {"Lorem ipsum"}
                </div>
                <div class="card-footer">
                    <div class="d-flex">
                        <LabSelect default={DefaultSelected::from(Id::from("lab"))} />
                        //<MachineSelect prototype="lab" default="lab" />
                        /*{
                            let num_modules_slots = 2; // todo
                            (0..num_modules_slots).map(|_| {
                                <ItemSelect prototype="module" />
                            })
                        }*/
                    </div>
                    <div class="d-flex justify-content-end">
                        <button type="button" class="btn btn-primary">
                            <yew_feather::PlusCircle />
                        </button>
                    </div>
                </div>
            </div>
            <div class="card w-100">
                <div class="card-header">
                    <div class="input-group mb-3">
                        <span class="input-group-text">{"Preset"}</span>
                        <select class="form-select" aria-label="Load technology preset">
                            <option selected={true}></option>
                            <option value="100pct">{"100 %"}</option>
                        </select>
                        <button class="btn btn-primary" type="button">{"Load"}</button>
                    </div>

                </div>
                <div class="card-body">

                </div>
                <div class="card-footer">
                    <div class="input-group mb-3">
                        <span class="input-group-text">{"Add technology"}</span>
                        <TechnologySelect />
                        <button class="btn btn-primary" type="button">
                            <yew_feather::PlusCircle />
                        </button>
                    </div>
                </div>
            </div>
        </form>
    }
}
