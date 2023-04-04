#include <string.h>
#include <stdlib.h>
#include <fs.h>

#include "internal_fs.h"

filesys_llnode *i_fs_head;

void fs_init()
{
  i_fs_head = 0;
}

filesys *fs_mount(char *name)
{
  if(!fs_valid_sys_name(name))
    return 0;

  filesys_llnode *newnode = calloc(1, sizeof(filesys_llnode));
  if(!newnode)
    return 0;

  filesys *newfs = &newnode->fs;
  if(!newfs)
    return 0;

  // Allocate root node
  // Not using fs_mkdir because it can complain about empty name
  fsnode *root = calloc(1, sizeof(fsnode));
  if(!root)
  {
    free(newnode);
    return 0;
  }

  // Setup root node
  root->type = FSNODE_DIR;
  root->fs = newfs;

  // Setup filesystem
  strcpy(newfs->name, name);
  newfs->root = root;

  newnode->next = i_fs_head;
  if(i_fs_head)
    i_fs_head->prev = newnode;
  i_fs_head = newnode;

  return newfs;
}

void fs_umount(filesys *fs)
{
  filesys_llnode *cfsn = i_fs_head;

  while(cfsn)
  {
    if(&cfsn->fs == fs)
      break;
    cfsn = cfsn->next;
  }

  if(cfsn->prev)
    cfsn->prev->next = cfsn->next;
  if(cfsn->next)
    cfsn->next->prev = cfsn->prev;

  // Call fs cleanup function
  if(fs->impl.fs_release)
    fs->impl.fs_release(fs);

  // Remove root dir(and all sub nodes)
  fs_rm(fs->root);

  free(fs);
}