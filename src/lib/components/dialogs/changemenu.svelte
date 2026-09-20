<script lang="ts">
  import * as Dialog from "$lib/components/ui/dialog/index.js";
  import { Button, buttonVariants } from "$lib/components/ui/button/index.js";
  import { Input } from "$lib/components/ui/input/index.js";
  import { api } from "$lib/api";
  import { IconTrash } from "@tabler/icons-svelte";

  // let {open= $bindable(false), image, name, tag} = $props();

  let {
    open = $bindable(false),
    id,
    image,
    name,
    tag,
    onchange,
  }: {
    open: boolean;
    id: string;
    image: string;
    name: string;
    tag: string;
    onchange?: () => Promise<void>;
  } = $props();

  let editName = $state("");
  let editTag = $state("");

  $effect(() => {
    if (open) {
      editName = name;
      editTag = tag;
    }
  });

  // Save updated title and tags
  async function handleSave() {
    try {
      await api.updateMeme(id, editName, [editTag]);
      if (onchange) await onchange();
      open = false;
    } catch (err) {
      console.error("Failed to update meme:", err);
    }
  }

  // Delete the meme from disk
  async function handleDelete() {
    try {
      await api.deleteMeme(id);
      if (onchange) await onchange();
      open = false;
    } catch (err) {
      console.error("Failed to delete meme:", err);
    }
  }
</script>

<Dialog.Root bind:open>
  <Dialog.Content class="sm:max-w-lg">
    <Dialog.Header>
      <Dialog.Title>Edit</Dialog.Title>
      <Dialog.Description>
        {name}
      </Dialog.Description>
    </Dialog.Header>

    <div class="flex flex-row items-center gap-4">
      <img
        src={image}
        alt={name}
        class="text-sm font-medium w-36 h-36 object-cover rounded-md border"
      />
      <div class="flex flex-col gap-2 flex-1">
        <label class="text-xs text-muted-foreground" for="meme-name">
          Meme Title
        </label>
        <Input id="meme-name" placeholder="Name" bind:value={editName} />

        <label class="text-xs text-muted-foreground" for="meme-tag">Tag</label>
        <Input
          id="meme-tag"
          placeholder="Tag (e.g. funny, reaction)"
          bind:value={editTag}
        />
      </div>
    </div>
    <Dialog.Footer
      class="flex flex-row justify-between items-center w-full pt-4"
    >
      <!-- Delete Button (Destructive) -->
      <Button variant="destructive" size="sm" onclick={handleDelete}>
        <IconTrash size={16} class="mr-1" />
        Delete
      </Button>

      <!-- Save & Close Buttons -->
      <div class="flex gap-2">
        <Dialog.Close class={buttonVariants({ variant: "ghost", size: "sm" })}>
          Cancel
        </Dialog.Close>
        <Button size="sm" onclick={handleSave}>Save Changes</Button>
      </div>
    </Dialog.Footer>
  </Dialog.Content>
</Dialog.Root>

