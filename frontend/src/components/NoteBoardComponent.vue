<template>
    <div class="note-board-overlay">
        <div class="note-board-header">
            <p class="header-text">Note board:</p>
            <button class="add-note" @click="openForm">+</button>
        </div>
        <div class="note-board">
            <div v-for="note in notes" :key="note.id" class="note">
                <p>{{ note.message }}</p>
                <small>{{ note.created_at.slice(0, 16) }}</small>
            </div>
        </div>
        <NoteFormComponent 
            v-if="showForm" 
            @submitNote="handleSubmitNote" 
            @close="closeForm" 
        />
    </div>
</template>

<script lang="ts">
import { defineComponent } from "vue";
import NoteFormComponent from "./NoteFormComponent.vue";

interface Note {
    id: number,
    message: string,
    created_at: string, //Something about time here. have to fix that
}

export default defineComponent ({
    name: "NoteBoardComponent",
    components: {
        NoteFormComponent,
    },
    data() {
        return {
            notes: [] as Note[],
            showForm: false,
        };
    },
    methods: {
        openForm() {
            this.showForm = true;
        },
        closeForm() {
            this.showForm = false;
        },
        async handleSubmitNote(message: string) {
            try {
                const response = await fetch("http://127.0.0.1:3001/note", {
                    method: "POST",
                    headers: {
                        "Content-Type": "application/json",
                    },
                    body: JSON.stringify({ message }),
                });

                if (!response.ok) {
                    const errorData = await response.json();
                    throw new Error(errorData.error || "Failed to create note");
                }

                const newNote: Note = await response.json();
                this.notes.push(newNote);
            } catch (error) {
                console.error("Error creating note:( - ", error)
            }
        },  
    },
    async created() {
        try {
            const response = await fetch("http://127.0.0.1:3001/notes");

            if (!response.ok) {
                throw new Error("Network response was not ok: " + response.statusText);
            } else if (response.status == 204) {
                console.log(204);
                
                const data: Note[] = [];
            } else {
                const data: Note[] = await response.json(); 
                this.notes = data;
            }
        } catch (error) {
        console.error("Error fetching note data: ", error);
        }
    },
});
</script>

<style scoped>
.note-board-overlay {
  width: 35vw;         /* 35% of the viewport width */
  margin: 0 auto;      /* Center horizontally */
  padding: 1rem;
  border: 1px solid #ccc;
}

.note-board-header {
  display: flex;
  align-items: center;
  justify-content: space-between;
  margin-bottom: 1rem;
}

.header-text {
  margin: 0;
}

.add-note {
  font-size: 1.5rem;
  padding: 0.2rem 0.5rem;
}

.note-board {
  margin-top: 1rem;
}

.note {
  padding: 0.5rem;
  margin-bottom: 0.5rem;
  border-bottom: 1px solid #eee;
}

</style>