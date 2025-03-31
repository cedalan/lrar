import {reactive} from "vue";

const sectionsUpdated = reactive({
    burnUpdated: false,
    dishwasherUpdated: false,
    notesUpdated: false,

})
function burnUpdated(){
    sectionsUpdated.burnUpdated = !sectionsUpdated.burnUpdated;
}
function dishwasherUpdated(){
    sectionsUpdated.dishwasherUpdated = !sectionsUpdated.dishwasherUpdated;
}
function notesUpdated(){
    sectionsUpdated.notesUpdated = !sectionsUpdated.notesUpdated;
}
export default {
    sectionsUpdated: sectionsUpdated,
    burnUpdated: burnUpdated,
    dishwasherUpdated,
    notesUpdated,
}