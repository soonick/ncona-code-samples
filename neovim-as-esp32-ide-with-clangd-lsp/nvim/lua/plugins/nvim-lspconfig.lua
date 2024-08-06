vim.api.nvim_create_autocmd('LspAttach', {
  desc = 'LSP actions',
  callback = function(event)
    -- Go to definition
    vim.keymap.set('n', 'gd', '<cmd>lua vim.lsp.buf.definition()<cr>', {buffer = event.buf})

    -- Return to previous location after going to definition
    vim.api.nvim_set_keymap('n', 'gb', '<C-t>', {})

    -- Go to definition in new tab
    vim.api.nvim_set_keymap('n', 'gdt', '<C-w><C-]><C-w>T', {})

    -- Code completion
    vim.api.nvim_set_keymap('i', '<C-Space>', '<C-x><C-o>', {})

    -- Don't open an empty buffer when triggering autocomplete
    vim.o.completeopt = 'menu'

    -- Show documentation for symbol
    vim.keymap.set('n', 'K', '<cmd>lua vim.lsp.buf.hover()<cr>', {buffer = event.buf})

    -- Format code
    vim.keymap.set('n', 'F', '<cmd>lua vim.lsp.buf.format()<cr>', {buffer = event.buf})

    -- Rename symbol
    vim.keymap.set('n', '3r', '<cmd>lua vim.lsp.buf.rename()<cr>', {buffer = event.buf})
  end
})

return {
  "neovim/nvim-lspconfig",
  config = function()
    require('lspconfig').clangd.setup {}
  end
}
