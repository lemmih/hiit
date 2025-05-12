#!/usr/bin/env bash

# Script to generate audio files for HIIT workout announcements
# Usage: ./audio_gen.sh

# Exit on error
set -e

# Exit on undefined variable
set -u

# Exit if any command in a pipe fails
set -o pipefail

# Check for required dependencies
for cmd in curl jq; do
    if ! command -v "$cmd" &>/dev/null; then
        echo "Error: Required command '$cmd' is not installed" >&2
        exit 1
    fi
done

VOICES=("freya" "vlad")

TEXTS=(
    "90/90 Hip Stretch"
    "Bicycle Crunches"
    "Burpees"
    "Calf Raises"
    "Crunches"
    "Downward to Upward Dog"
    "Froggy Glute Lifts"
    "Hammer Curls"
    "Inchworm"
    "Low Plank"
    "Lunge and Reach"
    "Lunges"
    "Modified Butterfly Sit"
    "Mountain Climbers"
    "Overhead Triceps"
    "Prepare"
    "Push Ups"
    "Rest"
    "Russian Twists"
    "Set Break"
    "Skull Crushers"
    "Squats"
    "Squat to Toe Touch"
    "Superman"
    "Three Two One"
    "Workout Complete"
    "Pigeon Pose"
)

# Function to generate filename from text and voice
generate_filename() {
    local text="$1"
    local voice="$2"
    # Convert to lowercase, replace spaces and punctuation with underscore
    local filename
    filename=$(echo "$text" | tr '[:upper:]' '[:lower:]' | tr '[:punct:][:space:]' '_' | tr --squeeze-repeats '_' | sed 's/_*$//')
    echo "${filename}_${voice}.mp3"
}

# Test generate_filename function
test_generate_filename() {
    local test_cases=(
        "Test Text:freya:test_text_freya.mp3"
        "Multiple   Spaces:vlad:multiple_spaces_vlad.mp3"
        "Punctuation!@#$%:freya:punctuation_freya.mp3"
        "Multiple???Punctuation:vlad:multiple_punctuation_vlad.mp3"
        "MiXeD cAsE:vlad:mixed_case_vlad.mp3"
        "90/90 Hip Stretch:freya:90_90_hip_stretch_freya.mp3"
    )

    local result
    for test in "${test_cases[@]}"; do
        IFS=':' read -r input voice expected <<<"$test"
        result=$(generate_filename "$input" "$voice")
        if [[ "$result" != "$expected" ]]; then
            echo "Test failed for: $input"
            echo "Expected: $expected"
            echo "Got: $result"
            exit 1
        fi
    done
}

# Run the tests
test_generate_filename

# Get the directory containing this script
SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
AUDIO_DIR="$SCRIPT_DIR/../public/audio"

# Check for missing audio files
missing_files=()

for voice in "${VOICES[@]}"; do
    for text in "${TEXTS[@]}"; do
        filename=$(generate_filename "$text" "$voice")
        if [[ ! -f "$AUDIO_DIR/$filename" ]]; then
            missing_files+=("$text:$voice")
        fi
    done
done

if [[ ${#missing_files[@]} -eq 0 ]]; then
    echo "All audio files are present"
    exit 0
fi

echo "Missing audio files:"
for pair in "${missing_files[@]}"; do
    IFS=':' read -r text voice <<<"$pair"
    echo "  - '$text' ($voice)"
done

# Function to get voice ID from ElevenLabs API
get_voice_id() {
    local voice="$1"
    local api_key="${ELEVENLABS_API_KEY}"
    local voice_id

    if [[ -z "$api_key" ]]; then
        echo "Error: ELEVENLABS_API_KEY environment variable not set" >&2
        exit 1
    fi

    # Call ElevenLabs API and get voice ID
    local response
    response=$(curl -s -H "xi-api-key: $api_key" \
        "https://api.elevenlabs.io/v2/voices?search=$voice")

    # Extract first voice_id from response using jq
    voice_id=$(echo "$response" | jq -r '.voices[0].voice_id')

    if [[ -z "$voice_id" || "$voice_id" == "null" ]]; then
        echo "Error: Could not find voice ID for '$voice'" >&2
        exit 1
    fi

    echo "$voice_id"
}

# Initialize associative array for voice ID mapping
declare -A voice_id_map

# Populate voice ID map for each voice
for voice in "${VOICES[@]}"; do
    voice_id_map[$voice]=$(get_voice_id "$voice")
done

# Function to generate MP3 file from text using ElevenLabs API
generate_audio() {
    local text="$1"
    local voice="$2"
    local output_file="$3"
    local api_key="${ELEVENLABS_API_KEY}"
    local voice_id="${voice_id_map[$voice]}"

    if [[ -z "$api_key" ]]; then
        echo "Error: ELEVENLABS_API_KEY environment variable not set" >&2
        exit 1
    fi

    if [[ -z "$voice_id" ]]; then
        echo "Error: No voice ID found for '$voice'" >&2
        exit 1
    fi

    echo "Generating audio for '$text' using voice '$voice'..." >&2

    # Call ElevenLabs API to generate audio
    curl -s -X POST \
        -H "xi-api-key: $api_key" \
        -H "Content-Type: application/json" \
        -d "{\"text\":\"$text\", \"model_id\":\"eleven_multilingual_v2\"}" \
        "https://api.elevenlabs.io/v1/text-to-speech/$voice_id?output_format=mp3_44100_128" \
        --output "$output_file"

    if [[ ! -f "$output_file" || ! -s "$output_file" ]]; then
        echo "Error: Failed to generate audio file" >&2
        exit 1
    fi

    echo "Successfully generated audio file: $output_file" >&2
}

# Function to generate and review audio files
generate_and_review_audio() {
    local text="$1"
    local voice="$2"
    local output_file="$3"

    # Skip if output file already exists
    if [[ -f "$output_file" ]]; then
        return 0
    fi

    # Create temporary file
    local temp_file
    temp_file=$(mktemp "${output_file}.XXXXXX")

    # Generate initial audio in temporary file
    generate_audio "$text" "$voice" "$temp_file"

    while true; do
        # Play the audio file
        afplay "$temp_file" 2>/dev/null || mpv --quiet "$temp_file" 2>/dev/null || mpg123 "$temp_file" 2>/dev/null || play "$temp_file" 2>/dev/null

        # Prompt for user input
        read -r -p "Audio review ([r]eplay/[n]ew/[y]es accept/[q]uit): " choice

        case "$choice" in
        r | R)
            continue
            ;;
        n | N)
            # Generate new audio
            generate_audio "$text" "$voice" "$temp_file"
            continue
            ;;
        y | Y)
            mv "$temp_file" "$output_file"
            return 0
            ;;
        q | Q)
            rm "$temp_file"
            exit 0
            ;;
        *)
            echo "Invalid choice. Please select r, n, y, or q" >&2
            ;;
        esac
    done
}

# Create audio directory if it doesn't exist
mkdir -p "$AUDIO_DIR"

# Generate and review missing audio files
for pair in "${missing_files[@]}"; do
    IFS=':' read -r text voice <<<"$pair"
    filename=$(generate_filename "$text" "$voice")
    output_file="$AUDIO_DIR/$filename"

    generate_and_review_audio "$text" "$voice" "$output_file"
done

echo "All missing audio files have been processed!"
