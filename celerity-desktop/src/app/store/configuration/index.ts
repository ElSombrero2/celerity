import { PayloadAction, createSlice } from "@reduxjs/toolkit";
import { Configuration } from "../../types/configuration";
import { Template } from "../../types/configuration/template";

const initialState: {configuration: Configuration | null, templates: Template[]} = { 
    configuration: null,
    templates: [], 
}

const configurationSlice = createSlice({
    name: 'configuration',
    initialState,
    reducers: {
        setConfiguration: (state, action: PayloadAction<Configuration>) => { 
            state.configuration = action.payload
        },
        setTemplates: (state, actions: PayloadAction<Template[]>) => {
            state.templates = actions.payload
        } 
    }
})

export const { setConfiguration, setTemplates } = configurationSlice.actions

export default configurationSlice.reducer