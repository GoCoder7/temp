<script lang="ts">
  import * as Card from "$lib/shad/ui/card";
  import type { Snippet } from 'svelte';

  const {
    title,
    description,
    content,
    footer
  } : {
    title?: string;
    description?: string;
    content: Snippet;
    footer?: Snippet;
  }
   = $props();
</script>

<Card.Root>
  {#if title || description}
    <Card.Header class="mb-4">
      {#if title}
        <Card.Title>{title}</Card.Title>
      {/if}
      {#if description}
        <Card.Description>{description}</Card.Description>
      {/if}
    </Card.Header>
    <hr />
  {/if}
  <hr />
  <Card.Content class="p-3">
    {@render content()}
  </Card.Content>
  {#if footer}
    <Card.Footer>
      {@render footer()}
    </Card.Footer>
  {/if}
</Card.Root>