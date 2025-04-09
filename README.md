# darchive - Discord Channel Archiver

A simple utility to archive Discord channels into browsable HTML and JSON formats. This tool downloads messages and their attachments from a specified Discord channel.

## Description

darchive connects to Discord's API to retrieve all messages from a channel and saves them along with their attachments. The output includes:
- A JSON file containing all message data
- An HTML file for easy browsing of the archived content
- All message attachments stored in an assets directory

## Usage

```
darchive <DISCORD_TOKEN> <CHANNEL_ID>
```

## Known Limitations

This is a rough implementation written quickly to archive a specific channel, so it has several shortcomings:

- **Minimal testing**: Only tested on one channel
- **Memory inefficient**: Loads all messages into memory before saving
- **No resume capability**: Cannot resume interrupted downloads
- **No handling for embeds**: Only captures text content and attachments
- **No filtering options**: Cannot specify date ranges or message limits

## Bot Permissions

- Discord bot token with message content intent enabled
- The bot must have access to the target channel

## Disclaimer

Licensed under the MIT License. Use at your own risk and ensure you have permission to archive channel content. Be mindful of Discord's Terms of Service regarding API usage.

THE SOFTWARE IS PROVIDED “AS IS”, WITHOUT WARRANTY OF ANY KIND, EXPRESS OR IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY, FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM, OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE SOFTWARE.
