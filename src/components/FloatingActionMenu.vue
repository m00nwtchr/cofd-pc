<template>
	<div class="fam">
		<div v-if="extended">
			<button
				v-for="(item, i) in items"
				:key="i"
				class="fab"
				:style="{ bottom: (i + 1) * (24 * 2 + 15) + 'px' }"
				@click="item.action"
			>
				<span v-if="item.icon">
					<font-awesome-icon :icon="item.icon" />
				</span>
				<span v-else>{{ item.name }}</span>
			</button>
		</div>

		<button v-if="items.length > 1" @click="extended = !extended" class="fab">
			<font-awesome-icon icon="bars"/>
		</button>
		<button
			v-else
			class="fab"
			@click="items[0].action"
		>
			<span v-if="items[0].icon" class="material-icons">
				<font-awesome-icon :icon="items[0].icon" />
			</span>
			<span v-else>{{ items[0].name }}</span>
		</button>
	</div>
</template>

<script lang="ts">
import { defineComponent, PropType } from "vue";

interface MenuItem {
	name?: string;
	icon?: string;
	action?: () => void;
}

export default defineComponent({
	name: "FloatingActionMenu",
	props: {
		"items": {
			required: true,
			type: Array as PropType<MenuItem[]>,
		}
	},
	data() {
		return {
			extended: false,
			// items: [
			// 	{
			// 		name: "Roll Selected",
			// 		icon: "casino",
			// 		action: () => {
			// 			alert("Huehuehue");
			// 		}
			// 	},
			// 	{
			// 		name: "Roll Selected",
			// 		action: () => {
			// 			alert("Huehuehue");
			// 		}
			// 	}
			// ] as MenuItem[]
		};
	}
});
</script>


<style lang="scss" scoped>
@use "sass:math";

$radius: 24px;

.fab {
	border-radius: $radius;

	width: $radius * 2;
	height: $radius * 2;

	position: fixed;

	right: 10px;
}

.fab svg {
	margin-top: math.div($radius, 8);
	width: $radius;
	height: $radius;	
}

.fam {
	position: fixed;

	right: $radius * 2+10;
	bottom: $radius * 2+10;

	background-color: red;

	z-index: 1;
}
</style>
