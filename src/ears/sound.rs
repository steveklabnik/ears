// The MIT License (MIT)
// 
// Copyright (c) 2013 Jeremy Letang (letang.jeremy@gmail.com)
// 
// Permission is hereby granted, free of charge, to any person obtaining a copy of
// this software and associated documentation files (the "Software"), to deal in
// the Software without restriction, including without limitation the rights to
// use, copy, modify, merge, publish, distribute, sublicense, and/or sell copies of
// the Software, and to permit persons to whom the Software is furnished to do so,
// subject to the following conditions:
// 
// The above copyright notice and this permission notice shall be included in all
// copies or substantial portions of the Software.
// 
// THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
// IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY, FITNESS
// FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE AUTHORS OR
// COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER LIABILITY, WHETHER
// IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM, OUT OF OR IN
// CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE SOFTWARE.

/*!
* Play Sounds easily.
*
* Simple class to play sound easily in 2 lines, Sounds are really ligth objects, 
* the sounds data are entirely load in memory and can be share between Sounds using the SoundData object.
*
* # Examples 
* ```Rust
* extern mod ears;
* use ears::Sound;
*
* fn main() -> () {
*    // Create a Sound whith the path of the sound file.
*    let snd = Sound::new(~"path/to/my/sound.ogg").unwrap();
*
*    // Play it   
*    snd.play();
*
*    // Wait until the sound is playing
*    while snd.is_playing() {}
* }
* ```
*/

use internal::*;
use sound_data::*;
use openal::{ffi, al};
use states::*;
use audio_controller::AudioController;

/// The Sound struct.
pub struct Sound {
    /// The internal OpenAl source identifier
    priv al_source  : u32,
    /// The SoundData associated to the Sound.
    priv sound_data : @SoundData
}

impl Sound {
    /**
    * Default constructor for Sound struct.
    *
    * Create a new struct and an associated SoundData.
    * 
    * # Argument
    * `path` - The path of the sound file to create the SoundData.
    *
    * # Return
    * An Option with Some(Sound) if the Sound is created properly, or None if un error has occured.
    */
    #[fixed_stack_segment] #[inline(never)]
    pub fn new(path : &str) -> Option<Sound> {
        match OpenAlData::check_al_context() {
            Ok(_)       => {},
            Err(err)    => { println!("{}", err); return None; }
        };

        let s_data = match SoundData::new(path) {
            Some(s_d)   => @s_d,
            None        => return None
        };

        Sound::new_with_data(s_data)
    }

    /**
    * Create a new struct with a SoundData to associate.
    * 
    * # Argument
    * `sound_data` - The sound_data to associate to the Sound.
    *
    * # Return
    * An Option with Some(Sound) if the Sound is created properly, or None if un error has occured.
    */
    #[fixed_stack_segment] #[inline(never)]
    pub fn new_with_data(sound_data : @SoundData) -> Option<Sound> {
        match OpenAlData::check_al_context() {
            Ok(_)       => {},
            Err(err)    => { println!("{}", err); return None; }
        };

        let mut source_id = 0;
        unsafe {
            // create the source
            ffi::alGenSources(1, &mut source_id);
            // set the buffer
            ffi::alSourcei(source_id, ffi::AL_BUFFER, sound_data.get_buffer() as i32);
        }

        // Check if there is OpenAL internal error
        match al::openal_has_error() {
            Some(err) => { println!("{}", err); return None; },
            None => {} 
        };

        Some(Sound {
            al_source   : source_id,
            sound_data  : sound_data
        })
    }

    /**
    * Get the sound datas.
    *
    * # Return
    * The SoundData associated to this Sound.
    */
    pub fn get_datas(&self) -> @SoundData {
        self.sound_data
    }
}

impl AudioController for Sound {
    /**
    * Play or resume the Sound.
    */
    #[fixed_stack_segment] #[inline(never)]
    fn play(&mut self) -> () {
        match OpenAlData::check_al_context() {
            Ok(_)       => {},
            Err(err)    => { println!("{}", err); return; }
        };

        unsafe {
             ffi::alSourcePlay(self.al_source);
        }
        match al::openal_has_error() {
            None => {},
            Some(err) => println!("{}", err)
        }
    }

    /**
    * Pause the Sound.
    */
    #[fixed_stack_segment] #[inline(never)]
    fn pause(&mut self) -> () {
        match OpenAlData::check_al_context() {
            Ok(_)       => {},
            Err(err)    => { println!("{}", err); return; }
        };

        unsafe {
            ffi::alSourcePause(self.al_source)
        }
    }

    /**
    * Stop the Sound.
    */
    #[fixed_stack_segment] #[inline(never)]
    fn stop(&mut self) -> () {
        match OpenAlData::check_al_context() {
            Ok(_)       => {},
            Err(err)    => { println!("{}", err); return; }
        };

        unsafe {
            ffi::alSourceStop(self.al_source)
        }
    }

    /**
    * Check if the Sound is playing or not.
    *
    * # Return
    * True if the Sound is playing, false otherwise.
    */
    fn is_playing(&self) -> bool {
        match self.get_state() {
            Playing     => true,
            _           => false
        }
    }

    /**
    * Get the current state of the Sound
    *
    * # Return
    * The state of the sound as a variant of the enum State
    */
    #[fixed_stack_segment] #[inline(never)]
    fn get_state(&self) -> State {
        match OpenAlData::check_al_context() {
            Ok(_)       => {},
            Err(err)    => { println!("{}", err); return Initial; }
        };

        let mut state : i32 = 0;
        unsafe {
            ffi::alGetSourcei(self.al_source, ffi::AL_SOURCE_STATE, &mut state);
        }
        match state {
            ffi::AL_INITIAL     => Initial,
            ffi::AL_PLAYING     => Playing,
            ffi::AL_PAUSED      => Paused,
            ffi::AL_STOPPED     => Stopped,
            _                   => unreachable!()
        }
        
    }
 
    /**
    * Set the volume of the Sound.
    *
    * A value of 1.0 means unattenuated. Each division by 2 equals an attenuation
    * of about -6dB. Each multiplicaton by 2 equals an amplification of about
    * +6dB.
    *
    * # Argument
    * * `volume` - The volume of the Sound, should be between 0. and 1. 
    */
    #[fixed_stack_segment] #[inline(never)]
    fn set_volume(&mut self, volume : f32) -> () {
        match OpenAlData::check_al_context() {
            Ok(_)       => {},
            Err(err)    => { println!("{}", err); return; }
        };
        unsafe {
            ffi::alSourcef(self.al_source, ffi::AL_GAIN, volume);
        }
    }

    /**
    * Get the volume of the Sound.
    *
    * # Return
    * The volume of the Sound between 0. and 1.
    */
    #[fixed_stack_segment] #[inline(never)]
    fn get_volume(&self) -> f32 {
        match OpenAlData::check_al_context() {
            Ok(_)       => {},
            Err(err)    => { println!("{}", err); return 0.; }
        };
        let mut volume : f32 = 0.;
        unsafe {
            ffi::alGetSourcef(self.al_source, ffi::AL_GAIN, &mut volume);
        }
        volume
    }

    /**
    * Set the minimal volume for a Sound.
    *
    * The minimum volume allowed for a source, after distance and cone attenation is
    * applied (if applicable).
    *
    * # Argument
    * * `min_volume` - The new minimal volume of the Sound should be between 0. and 1. 
    */
    #[fixed_stack_segment] #[inline(never)]
    fn set_min_volume(&mut self, min_volume : f32) -> () {
        match OpenAlData::check_al_context() {
            Ok(_)       => {},
            Err(err)    => { println!("{}", err); return; }
        };
        unsafe {
            ffi::alSourcef(self.al_source, ffi::AL_MIN_GAIN, min_volume);
        }
    }

    /**
    * Get the minimal volume of the Sound.
    *
    * # Return
    * The minimal volume of the Sound between 0. and 1.
    */
    #[fixed_stack_segment] #[inline(never)]
    fn get_min_volume(&self) -> f32 {
        match OpenAlData::check_al_context() {
            Ok(_)       => {},
            Err(err)    => { println!("{}", err); return 0.; }
        };
        let mut volume : f32 = 0.;
        unsafe {
            ffi::alGetSourcef(self.al_source, ffi::AL_MIN_GAIN, &mut volume);
        }
        volume
    }

    /**
    * Set the maximal volume for a Sound.
    *
    * The maximum volume allowed for a sound, after distance and cone attenation is
    * applied (if applicable).
    *
    * # Argument
    * * `max_volume` - The new maximal volume of the Sound should be between 0. and 1. 
    */
    #[fixed_stack_segment] #[inline(never)]
    fn set_max_volume(&mut self, max_volume : f32) -> () {
        match OpenAlData::check_al_context() {
            Ok(_)       => {},
            Err(err)    => { println!("{}", err); return; }
        };
        unsafe {
            ffi::alSourcef(self.al_source, ffi::AL_MAX_GAIN, max_volume);
        }
    }

    /**
    * Get the maximal volume of the Sound.
    *
    * # Return
    * The maximal volume of the Sound between 0. and 1.
    */
    #[fixed_stack_segment] #[inline(never)]
    fn get_max_volume(&self) -> f32 {
        match OpenAlData::check_al_context() {
            Ok(_)       => {},
            Err(err)    => { println!("{}", err); return 0.; }
        };
        let mut volume : f32 = 0.;
        unsafe {
            ffi::alGetSourcef(self.al_source, ffi::AL_MAX_GAIN, &mut volume);
        }
        volume
    }

    /**
    * Set the Sound looping or not
    *
    * The default looping is false.
    *
    * # Arguments
    * `looping` - The new looping state.
    */
    #[fixed_stack_segment] #[inline(never)]
    fn set_looping(&mut self, looping : bool) -> () {
        match OpenAlData::check_al_context() {
            Ok(_)       => {},
            Err(err)    => { println!("{}", err); return; }
        };
        unsafe {
            match looping {
                true    => ffi::alSourcei(self.al_source, ffi::AL_LOOPING, ffi::ALC_TRUE as i32),
                false   => ffi::alSourcei(self.al_source, ffi::AL_LOOPING, ffi::ALC_FALSE as i32)
            };
        }
    }

    /**
    * Check if the Sound is looping or not
    *
    * # Return
    * True if the Sound is looping, false otherwise.
    */
    #[fixed_stack_segment] #[inline(never)]
    fn is_looping(&self) -> bool {
        match OpenAlData::check_al_context() {
            Ok(_)       => {},
            Err(err)    => { println!("{}", err); return false; }
        };
        let mut boolean = 0; 
        unsafe {
            ffi::alGetSourcei(self.al_source, ffi::AL_LOOPING, &mut boolean);
        }
        match boolean as i8 {
            ffi::ALC_TRUE       => true,
            ffi::ALC_FALSE      => false,
            _                   => unreachable!()
        }
    }

    /**
    * Set the pitch of the source.
    * 
    * A multiplier for the frequency (sample rate) of the source's buffer.
    *
    * Default pitch is 1.0.
    * 
    * # Argument
    * * `new_pitch` - The new pitch of the sound in the range [0.5 - 2.0]
    */
    #[fixed_stack_segment] #[inline(never)]
    fn set_pitch(&mut self, pitch : f32) -> () {
        match OpenAlData::check_al_context() {
            Ok(_)       => {},
            Err(err)    => { println!("{}", err); return; }
        };

        unsafe {
            ffi::alSourcef(self.al_source, ffi::AL_PITCH, pitch)
        }
    }

    /**
    * Set the pitch of the source.
    * 
    * # Return
    * The pitch of the sound in the range [0.5 - 2.0]
    */
    #[fixed_stack_segment] #[inline(never)]
    fn get_pitch(&self) -> f32 {
        match OpenAlData::check_al_context() {
            Ok(_)       => {},
            Err(err)    => { println!("{}", err); return 0.; }
        };

        let mut pitch = 0.;
        unsafe {
            ffi::alGetSourcef(self.al_source, ffi::AL_PITCH, &mut pitch)
        }
        pitch
    }

    /**
    * Set the position of the sound relative to the listener or absolute.
    *
    * Default position is absolute.
    *
    * # Argument
    * `relative` - True to set sound relative to the listener false to set the sound position absolute.
    */
    #[fixed_stack_segment] #[inline(never)]
    fn set_relative(&mut self, relative : bool) -> () {
        match OpenAlData::check_al_context() {
            Ok(_)       => {},
            Err(err)    => { println!("{}", err); return; }
        };
        unsafe {
            match relative {
                true    => ffi::alSourcei(self.al_source, ffi::AL_SOURCE_RELATIVE, ffi::ALC_TRUE as i32),
                false   => ffi::alSourcei(self.al_source, ffi::AL_SOURCE_RELATIVE, ffi::ALC_FALSE as i32)
            };
        }
    }

    /**
    * Is the sound relative to the listener or not ?
    *
    * # Return
    * True if the sound is relative to the listener false otherwise
    */
    #[fixed_stack_segment] #[inline(never)]
    fn is_relative(&mut self) -> bool {
        match OpenAlData::check_al_context() {
            Ok(_)       => {},
            Err(err)    => { println!("{}", err); return false; }
        };

        let mut boolean = 0; 
        unsafe {
            ffi::alGetSourcei(self.al_source, ffi::AL_SOURCE_RELATIVE, &mut boolean);
        }
        match boolean as i8 {
            ffi::ALC_TRUE       => true,
            ffi::ALC_FALSE      => false,
            _                   => unreachable!()
        }
    }

    /**
    * Set the Sound location in three dimensional space.
    *
    * OpenAL, like OpenGL, uses a right handed coordinate system, where in a
    * frontal default view X (thumb) points right, Y points up (index finger), and
    * Z points towards the viewer/camera (middle finger). 
    * To switch from a left handed coordinate system, flip the sign on the Z
    * coordinate.
    *
    * Default position is [0., 0., 0.]. 
    *
    * # Argument
    * * `position` - A three dimensional vector of f32 containing the position of the listener [x, y, z].
    */
    #[fixed_stack_segment] #[inline(never)]
    fn set_position(&mut self, position : [f32, ..3]) -> () {
        match OpenAlData::check_al_context() {
            Ok(_)       => {},
            Err(err)    => { println!("{}", err); return; }
        };
        unsafe {
            ffi::alSourcefv(self.al_source, ffi::AL_POSITION, &position[0]);
        }
    }

    /**
    * Get the position of the Sound in three dimensional space.
    *
    * # Return
    * A three dimensional vector of f32 containing the position of the listener [x, y, z].
    */
    #[fixed_stack_segment] #[inline(never)]
    fn get_position(&self) -> [f32, ..3] {
        match OpenAlData::check_al_context() {
            Ok(_)       => {},
            Err(err)    => { println!("{}", err); return [0., ..3]; }
        };
        let mut position : [f32, ..3] = [0., ..3];
        unsafe {
            ffi::alGetSourcefv(self.al_source, ffi::AL_POSITION, &mut position[0]);
        }
        position
    }

    /**
    * Set the direction of the Sound.
    *
    * Specifies the current direction in local space.
    *
    * The default direction is: [0., 0., 0.]
    *
    * # Argument
    * `direction` - The new direction of the Sound.
    */
    #[fixed_stack_segment] #[inline(never)]
    fn set_direction(&mut self, direction : [f32, ..3]) -> () {
        match OpenAlData::check_al_context() {
            Ok(_)       => {},
            Err(err)    => { println!("{}", err); return; }
        };
        unsafe {
            ffi::alSourcefv(self.al_source, ffi::AL_DIRECTION, &direction[0]);
        }
    }

    /**
    * Get the direction of the Sound.
    *
    * # Return
    * The current direction of the Sound.
    */
    #[fixed_stack_segment] #[inline(never)]
    fn get_direction(&self)  -> [f32, ..3] {
        match OpenAlData::check_al_context() {
            Ok(_)       => {},
            Err(err)    => { println!("{}", err); return [0., ..3]; }
        };
        let mut direction : [f32, ..3] = [0., ..3];
        unsafe {
            ffi::alGetSourcefv(self.al_source, ffi::AL_DIRECTION, &mut direction[0]);
        }
        direction
    }

    /**
    * Set the maximum distance of the Sound.
    *
    * The distance above which the source is not attenuated any further with a
    * clamped distance model, or where attenuation reaches 0.0 gain for linear
    * distance models with a default rolloff factor.
    * 
    * The default maximum distance is +inf.
    *
    * # Argument
    * `max_distance` - The new maximum distance in the range [0., +inf]
    */
    #[fixed_stack_segment] #[inline(never)]
    fn set_max_distance(&mut self, max_distance : f32) -> () {
        match OpenAlData::check_al_context() {
            Ok(_)       => {},
            Err(err)    => { println!("{}", err); return; }
        };
        unsafe {
            ffi::alSourcef(self.al_source, ffi::AL_MAX_DISTANCE, max_distance);
        }
    }

    /**
    * Get the maximum distance of the Sound.
    *
    * # Return
    * The maximum distance of the Sound in the range [0., +inf]
    */
    #[fixed_stack_segment] #[inline(never)]
    fn get_max_distance(&self) -> f32 {
        match OpenAlData::check_al_context() {
            Ok(_)       => {},
            Err(err)    => { println!("{}", err); return 0.; }
        };
        let mut max_distance = 0.;
        unsafe {
            ffi::alGetSourcef(self.al_source, ffi::AL_MAX_DISTANCE, &mut max_distance);
        }
        max_distance
    }

    /**
    * Set the reference distance of the Sound.
    *
    * The distance in units that no attenuation occurs.
    * At 0.0, no distance attenuation ever occurs on non-linear attenuation models.
    *
    * The default distance reference is 1.
    *
    * # Argument
    * * `ref_distance` - The new reference distance of the Sound.
    */
    #[fixed_stack_segment] #[inline(never)]
    fn set_reference_distance(&mut self, ref_distance : f32) -> () {
        match OpenAlData::check_al_context() {
            Ok(_)       => {},
            Err(err)    => { println!("{}", err); return; }
        };
        unsafe {
            ffi::alSourcef(self.al_source, ffi::AL_REFERENCE_DISTANCE, ref_distance);
        }
    }

    /**
    * Get the reference distance of the Sound.
    *
    * # Return
    * The current reference distance of the Sound.
    */
    #[fixed_stack_segment] #[inline(never)]
    fn get_reference_distance(&self) -> f32 {
        match OpenAlData::check_al_context() {
            Ok(_)       => {},
            Err(err)    => { println!("{}", err); return 1.; }
        };
        let mut ref_distance = 0.;
        unsafe {
            ffi::alGetSourcef(self.al_source, ffi::AL_REFERENCE_DISTANCE, &mut ref_distance);
        }
        ref_distance
    }

    /**
    * Set the attenuation of a Sound.
    *
    * Multiplier to exaggerate or diminish distance attenuation.
    * At 0.0, no distance attenuation ever occurs.
    *
    * The default attenuation is 1.
    *
    * # Arguments
    * `attenuation` - The new attenuation for the sound in the range [0., 1.].
    */
    #[fixed_stack_segment] #[inline(never)]
    fn set_attenuation(&mut self, attenuation : f32) -> () {
        match OpenAlData::check_al_context() {
            Ok(_)       => {},
            Err(err)    => { println!("{}", err); return; }
        };
        unsafe {
            ffi::alSourcef(self.al_source, ffi::AL_ROLLOFF_FACTOR, attenuation);
        }
    }

    /**
    * Get the attenuation of a Sound.
    *
    * # Return
    * The current attenuation for the sound in the range [0., 1.].
    */
    #[fixed_stack_segment] #[inline(never)]
    fn get_attenuation(&self) -> f32 {
        match OpenAlData::check_al_context() {
            Ok(_)       => {},
            Err(err)    => { println!("{}", err); return 1.; }
        };
        let mut attenuation = 0.;
        unsafe {
            ffi::alGetSourcef(self.al_source, ffi::AL_ROLLOFF_FACTOR, &mut attenuation);
        }
        attenuation
    }

}

#[unsafe_destructor]
impl Drop for Sound {
    /**
    * Destroy all the resources attached to the Sound.
    */
    #[fixed_stack_segment] #[inline(never)]
    fn drop(&mut self) -> () {
        unsafe {
            ffi::alDeleteSources(1, &mut self.al_source);            
        }
    }
}