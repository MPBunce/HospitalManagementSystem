<template>
    <div>
        <div class='static z-40'>
            <nav class="fixed min-w-full container mx-auto p-6 bg-green-200 shadow top-0 left-0 right-0">

                <div class="flex items-stretch justify-between">

                    <div class="pl-12 pt-2 text-3xl font-extrabold text-frostfour">
                        <NuxtLink to="/" @click="login = false">Hospital.rs</NuxtLink>
                    </div>

                    <div class="hidden pt-2 space-x-6 lg:flex pr-14">

                        <NuxtLink to="/staff">Staff</NuxtLink>
                        <NuxtLink to="/supplies">Supplies</NuxtLink>
                        <NuxtLink to="/staff">Hospital Info</NuxtLink>

                    </div>

                    <button
                        @click="login = !login"
                        id="menu-btn"
                        class="block hamburger lg:hidden focus:outline-none"
                        >
                        MenuBtn
                    </button>

                </div>

                <div  class="lg:hidden">
                    


                    <div v-if="login" id="hiddenMenu" class="h-screen w-screen flex flex-col items-center pr-12 ">
                        
                        <NuxtLink class="my-12" to="/staff" @click="login = !login">Staff</NuxtLink>
                        <NuxtLink class="my-12" to="/supplies" @click="login = !login">Supplies</NuxtLink>
                        <NuxtLink class="my-12" to="/staff" @click="login = !login">Hospital Info</NuxtLink>

                    </div>

                    <div v-else>

                    </div>

                </div>
                

            </nav>
        </div>
        

        <div class="mt-40 mx-20">
            <slot />

        </div>

    </div>

</template>

<script>

    export default {
        data() {
            return {
                login: false,
            };
        }
    };
  
</script>
