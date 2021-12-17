<template>
    <section class="loading">
        <div class="lds-dual-ring" v-if="isLoading"></div>
    </section>
    <div class="loading-background" v-if="isLoading"></div>
    <main class="wrapper">
        <h1>TSV Hürup Adventskalender</h1>
        <section id="winners">
            <h2 v-if="winners.length">Folgende Tickets haben etwas gewonnen:</h2>
            <div v-for="entry in winners">
                <ul class="numbers-list">
                    <a :href="entry.url" target="_blank" class="numbers-list__link">
                        <li v-for="number in entry.numbers" class="numbers-list__item">
                            <p class="numbers-list__winner">{{ number.replace('#', '') }}</p>
                            <p class="numbers-list__winner-hint">Klicken, um den Gewinn einzusehen</p>
                        </li>
                    </a>
                </ul>
            </div>
        </section>
        <label for="newNumber">Gib eine neue Losnummer ein</label>
        <section class="new-number">
            <input v-model="newNumber" type="tel" class="new-number__input"/>
            <button @click="addNumber" class="button">Hinzufügen</button>
        </section>
        <h2>Deine Lose</h2>
        <ul name="numbers" class="numbers-list">
            <li v-for="number in numbers.slice().reverse()" class="numbers-list__item">
                <p class="numbers-list__winner">{{ number.replace('#', '') }}</p>
                <button @click="removeNumber(number)" class="numbers-list__item-remove">X</button>
            </li>
        </ul>
        <button @click="checkNumbers" class="button">Nummern überprüfen</button>
    </main>
</template>

<script lang="ts">
import { defineComponent } from 'vue';

export default defineComponent({
    data() {
        return {
          isLoading: false,
          newNumber: '0000',
          numbers: new Array<string>(),
          winners: new Array<{ url: string, numbers: string[] }>(),
        };
    },

    created() {
        let storageNumbers: string | null = localStorage.getItem('numbers');
        if (storageNumbers) {
            this.numbers = JSON.parse(storageNumbers);
        } else {
            localStorage.setItem('numbers', JSON.stringify(this.numbers));
        }
    },
    methods: {
        addNumber() {
            let candidate = '#' + this.newNumber;
            if (!this.numbers.includes(candidate)) {
                this.numbers.push(candidate);
                localStorage.setItem('numbers', JSON.stringify(this.numbers));
                this.newNumber = '0000';
            }
        },
        isValidNumber(number: string): boolean {
            const matches = number.match('/[0-9]{4}/');
            return matches ? matches.length > 0 : false;
        },
        removeNumber(number: string) {
            const index = this.numbers.indexOf(number);
            if (index > -1) {
                this.numbers.splice(index, 1);
            }
            localStorage.setItem('numbers', JSON.stringify(this.numbers));
        },
        async checkNumbers() {
            this.isLoading = true;
            this.winners = await this.fetchWinners();
            this.isLoading = false;
            window.scrollTo(0, 0);
        },
        async fetchWinners() {
            const url = 'http://localhost:7777/winners';
            const response = await fetch(url, {
                method: 'POST',
                headers: {
                    'Content-Type': 'application/json',
                },
                body: JSON.stringify({ numbers: this.numbers }),
            });

            return response.json();
        },
        isWinningNumber(number: string): boolean {
            if(!this.winners) {
                return false;
            }
            console.log(this.winners.filter((entry) => {
                return entry.numbers.includes(number);
            }));
            return true;
        },
    }
});
</script>

<style>
    html {
        scroll-behavior: smooth;
    }

    body {
        margin: 0;
        width: 100%;
        height: 100%;
    }

    .loading {
        position: fixed;
        z-index: 100;
        left: 50%;
        top: 50%;
    }

    .loading-background {
        position: fixed;
        top: -10%;
        width: 10000px;
        height: 10000px;
        z-index: 90;
        background-color: rgba(255, 255, 255, 0.5);
    }

    .lds-dual-ring {
        display: inline-block;
        width: 80px;
        height: 80px;
    }

    .lds-dual-ring:after {
        content: " ";
        display: block;
        width: 64px;
        height: 64px;
        margin: 8px 8px 8px -32px;
        border-radius: 50%;
        border: 6px solid black;
        border-color: black transparent black transparent;
        animation: lds-dual-ring 1.2s linear infinite;
    }

    @keyframes lds-dual-ring {
        0% {
            transform: rotate(0deg);
        }
        100% {
            transform: rotate(360deg);
        }
    }

    .wrapper {
        width: 90%;
        margin: 2rem auto;
    }

    .numbers-list {
        list-style: none;
        display: grid;
        grid-auto-flow: row;
        grid-gap: 1rem;
        padding: 0;
        margin: 2.5rem 0;
    }

    .numbers-list__item {
        display: flex;
        flex-direction: row;
        justify-content: center;
        background-color: #bb1e40;
        padding: 3rem 4rem;
        box-shadow: 3px 3px 3px #701226;
        position: relative;
    }

    .numbers-list__item-input {
        width: 5rem;
        text-align: center;
        padding: 1rem 0.5rem;
        font-weight: 800;
        font-size: 1.5rem;
    }

    .numbers-list__item-remove {
        position: absolute;
        top: 1rem;
        right: 1rem;
        background-color: #701226;
        color: white;
        border: none;
        padding: 0.5rem 0.75rem;
        text-align: center;
        font-weight: 800;
    }

    .numbers-list__link {
        text-decoration: none;
        color: black;
    }

    .numbers-list__winner {
        background: white;
        width: 5rem;
        text-align: center;
        padding: 1rem 0.5rem;
        font-weight: 800;
        font-size: 1.5rem;
    }

    .numbers-list__winner-hint {
        position: absolute;
        bottom: 0.5rem;
        font-weight: 800;
        font-size: 1.25rem;
        color: white;
    }

    .new-number {
        display: grid;
        grid-template-columns: 5rem auto;
        grid-gap: 2rem;
        margin: 1rem 0 2.5rem;
    }

    .new-number__input {
        text-align: center;
    }

    .button {
        background-color: #701226;
        color: white;
        border: none;
        font-weight: 800;
        padding: 1rem 1.5rem;
        width: 100%;
        cursor: pointer;
        transition: .5s ease-out;
    }

    .button:hover {
        background-color: #bb1e40;
    }

    @media screen and (min-width: 768px) {
        .numbers-list {
            grid-template-columns: auto auto;
        }

        .button {
            width: max-content;
        }
    }

    @media screen and (min-width: 1280px) {
        .numbers-list {
            grid-template-columns: auto auto auto;
        }

        .button {
            width: max-content;
        }
    }
</style>
