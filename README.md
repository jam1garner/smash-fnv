# smash-fnv

A Rust library for reading and writing `sound_volume_fighter_num_table.fnv` files from Super Smash Bros. for Nintendo 3DS and Wii U and Super Smash Bros. Ultimate. Not to be confused with [Fowler–Noll–Vo](https://en.wikipedia.org/wiki/Fowler%E2%80%93Noll%E2%80%93Vo_hash_function).

```
cargo add smash-fnv
```

## fnv_yaml

A command-line program for creating and editing `sound_volume_fighter_num_table.fnv` files using YAML. Drag and drop a `sound_volume_fighter_num_table.fnv` file onto the executable to create a YAML file. Drag and drop a properly structured YAML file onto the executable to create a `sound_volume_fighter_num_table.fnv` file. YAML files are text files, so they can be viewed and edited in any text editor.

Sample output from a `sound_volume_fighter_num_table.fnv` file:

```yaml
entries:
- fighter_num: 2
  volume:
    other: 0.6
    sound_attr: 0.2
    se_fighter_step: 0.3
    se_fighter_landing: 0.3
    se_collision_step: 0.3
    se_collision_landing: 0.3
    se_stage: 0.0
    bgm: 0.1
- fighter_num: 4
  volume:
    other: 0.0
    sound_attr: 0.0
    se_fighter_step: 0.0
    se_fighter_landing: 0.0
    se_collision_step: 0.0
    se_collision_landing: 0.0
    se_stage: -0.5
    bgm: 0.0
```

### Usage

The latest prebuilt binary for Windows is available in [Releases](https://github.com/jam1garner/smash-fnv/releases/latest).

`fnv_yaml <input> [output]`<br>
`fnv_yaml sound_volume_fighter_num_table.fnv sound_volume_fighter_num_table.yaml`<br>
`fnv_yaml sound_volume_fighter_num_table.yaml sound_volume_fighter_num_table.fnv`<br>
