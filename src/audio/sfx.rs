use bevy::audio::*;
use bevy::prelude::*;
use rand::prelude::*;
use std::sync::Arc;

#[derive(Message)]
pub enum AudioEvent {
    Shoot,
    Kill,
    Pickup,
    LevelUp,
    GameOver,
    Damage,
}

#[derive(Resource, Clone)]
pub struct SoundEffects {
    pub shoot: Handle<AudioSource>,
    pub kill: Handle<AudioSource>,
    pub pickup: Handle<AudioSource>,
    pub level_up: Handle<AudioSource>,
    pub game_over: Handle<AudioSource>,
    pub damage: Handle<AudioSource>,
}

fn make_tone(
    assets: &mut Assets<AudioSource>,
    freq: f32,
    dur: f32,
    vol: f32,
) -> Handle<AudioSource> {
    let sr = 44100;
    let n = (sr as f32 * dur) as usize;
    let mut samples = Vec::with_capacity(n);
    for i in 0..n {
        let t = i as f32 / sr as f32;
        let v = (t * freq * std::f32::consts::TAU).sin() * vol;
        let env = (1.0 - (i as f32 / n as f32)).min(1.0);
        samples.push((v * env * i16::MAX as f32) as i16);
    }
    let bytes: Arc<[u8]> = encode_wav_i16(&samples, sr).into();
    assets.add(AudioSource { bytes })
}

fn make_noise(assets: &mut Assets<AudioSource>, dur: f32, vol: f32) -> Handle<AudioSource> {
    let sr = 44100;
    let n = (sr as f32 * dur) as usize;
    let mut samples = Vec::with_capacity(n);
    let mut rng = rand::rng();
    for i in 0..n {
        let v = (rng.gen::<f32>() * 2.0 - 1.0) * vol;
        let env = (1.0 - (i as f32 / n as f32)).min(1.0);
        samples.push((v * env * i16::MAX as f32) as i16);
    }
    let bytes: Arc<[u8]> = encode_wav_i16(&samples, sr).into();
    assets.add(AudioSource { bytes })
}

fn encode_wav_i16(samples: &[i16], sample_rate: u32) -> Vec<u8> {
    let channels: u16 = 1;
    let bits: u16 = 16;
    let data_size = samples.len() as u32 * 2;
    let mut w = Vec::new();
    w.extend_from_slice(b"RIFF");
    w.extend_from_slice(&(36 + data_size).to_le_bytes());
    w.extend_from_slice(b"WAVEfmt ");
    w.extend_from_slice(&16u32.to_le_bytes());
    w.extend_from_slice(&1u16.to_le_bytes());
    w.extend_from_slice(&channels.to_le_bytes());
    w.extend_from_slice(&sample_rate.to_le_bytes());
    w.extend_from_slice(&(sample_rate * channels as u32 * (bits / 8) as u32).to_le_bytes());
    w.extend_from_slice(&(channels * (bits / 8)).to_le_bytes());
    w.extend_from_slice(&bits.to_le_bytes());
    w.extend_from_slice(b"data");
    w.extend_from_slice(&data_size.to_le_bytes());
    for &s in samples {
        w.extend_from_slice(&s.to_le_bytes());
    }
    w
}

pub fn setup_audio(mut commands: Commands, mut assets: ResMut<Assets<AudioSource>>) {
    commands.insert_resource(SoundEffects {
        shoot: make_tone(&mut assets, 440.0, 0.08, 0.3),
        kill: make_noise(&mut assets, 0.15, 0.3),
        pickup: make_tone(&mut assets, 880.0, 0.1, 0.2),
        level_up: make_tone(&mut assets, 660.0, 0.3, 0.3),
        game_over: make_tone(&mut assets, 110.0, 0.8, 0.4),
        damage: make_noise(&mut assets, 0.08, 0.2),
    });
}

pub fn play_audio_events(
    mut events: MessageReader<AudioEvent>,
    sfx: Res<SoundEffects>,
    mut commands: Commands,
) {
    for event in events.read() {
        let h = match event {
            AudioEvent::Shoot => &sfx.shoot,
            AudioEvent::Kill => &sfx.kill,
            AudioEvent::Pickup => &sfx.pickup,
            AudioEvent::LevelUp => &sfx.level_up,
            AudioEvent::GameOver => &sfx.game_over,
            AudioEvent::Damage => &sfx.damage,
        };
        commands.spawn((AudioPlayer::new(h.clone()), PlaybackSettings::DESPAWN));
    }
}
