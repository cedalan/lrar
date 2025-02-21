<template>
    <div class="modal-overlay">
      <div class="modal-content">
        <h3>Add a New Note</h3>
        <textarea v-model="message" placeholder="Type your note here..."></textarea>
        <div class="buttons">
          <button @click="submitNote">Submit</button>
          <button @click="close">Cancel</button>
        </div>
      </div>
    </div>
  </template>
  
  <script lang="ts">
  import { defineComponent, ref } from "vue";
  
  export default defineComponent({
    name: "NoteFormComponent",
    emits: ["submitNote", "close"],
    setup(_, { emit }) {
      const message = ref("");
  
      const submitNote = () => {
        if (message.value.trim()) {
          // Emit the note message to the parent component
          emit("submitNote", message.value.trim());
          message.value = "";
        }
      };
  
      const close = () => {
        emit("close");
      };
  
      return {
        message,
        submitNote,
        close,
      };
    },
  });
  </script>
  
  <style scoped>
  .modal-overlay {
    position: fixed;
    top: 0;
    left: 0;
    width: 100%;
    height: 100%;
    background-color: rgba(0, 0, 0, 0.5);
    display: flex;
    align-items: center;
    justify-content: center;
  }
  
  .modal-content {
    background-color: #fff;
    padding: 1rem;
    border-radius: 5px;
    width: 300px;
  }
  
  textarea {
    width: 100%;
    height: 100px;
    resize: none;
  }
  
  .buttons {
    display: flex;
    justify-content: space-between;
    margin-top: 1rem;
  }
  </style>
  